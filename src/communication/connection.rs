use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt, };
use tokio::{net::TcpStream, sync::{watch}};
use tokio_tungstenite::{connect_async, tungstenite::{handshake::client::Request, http::Uri, protocol::Message, Error}, MaybeTlsStream, WebSocketStream};

use anyhow::{anyhow, Result};
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::{error, info};
use tracing::log::trace;
use crate::client::hotel;
use crate::client::session::Session;
use crate::communication::{incoming, packet};
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;

pub struct Connector {
    pub ws_link: String,
    pub origin: String,
}

impl Connector {
    pub fn new(ws_link: String, origin: String) -> Self {
        Connector{ws_link, origin }
    }

    pub async fn connect(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let uri = Uri::try_from(self.ws_link.clone())?;

        let request = Request::builder()
            .uri(uri)
            .header("Origin", self.origin.clone())
            .body(())
            .expect("Failed to build request");

        let connect_future = connect_async(request);

        match timeout(Duration::from_secs(2), connect_future).await {
            Ok(Ok((ws_stream, _))) => Ok(ws_stream),
            Ok(Err(e)) => Err(anyhow!("WebSocket connection error: {}", e)),
            Err(_) => Err(anyhow!("WebSocket connection timeout")),
        }
    }
}

pub struct Receiver {
    event_handler: Arc<incoming::message::Handler>
}

impl Receiver {
    pub fn new(event_handler: Arc<incoming::message::Handler>) -> Self {
        Receiver{ event_handler }
    }

    async fn handle_client_message(&self, msg: Message, session: Arc<Session>) -> Result<()> {
        if msg.is_close() {
            return Err(anyhow!("Websocket connection is closed"));
        }

        let _ = self.event_handler.handle(session.clone(), msg.into_data()).await?;

        Ok(())
    }

    pub async fn receive(&self, mut reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>, session: Arc<Session>, cancellation: watch::Sender<bool>) -> Result<()> {
        let mut kill_sig = session.kill_sig_rx.clone();

        loop {
            tokio::select! {
                data = reader.next() => {
                    if let Some(Ok(msg)) = data {
                        if let Err(err) = self.handle_client_message(msg, session.clone()).await {
                            cancellation.send(true)?;

                            error!("Reader closed, shutdown signal sent to listeners. Reason: {:?}", err);
                            break;
                        }

                        continue;
                    }

                    cancellation.send(true)?;
                    break;
                },

                _ = kill_sig.changed() => {
                    if *kill_sig.borrow() {
                        info!("Kill sig triggered, reader is now closed");
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

pub struct Sender {}

impl Sender {
    pub fn new() -> Self {
        Sender {}
    }

    async fn send(&self, mut writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, session: Arc<Session>, mut ch: mpsc::Receiver<Message>, mut shutdown_rx: watch::Receiver<bool>) {
        let mut kill_sig = session.kill_sig_rx.clone();

        loop {
            tokio::select! {
                data = ch.recv() => {
                    if let Some(msg) = data {
                        if let Err(err) = writer.send(msg).await {
                            error!("An error occurred while trying to send data to the websocket: {:?}", err);
                            continue;
                        }
                    }
                },

                _ = kill_sig.changed() => {
                    if *kill_sig.borrow() {
                        info!("Kill sig triggered, reader is now closed");
                        break;
                    }
                }

                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        info!("Sender closed, shutdown channel has been triggered");
                        break;
                    }
                }
            }
        }
    }
}

pub struct Handler {
    packet_receiver: Arc<Receiver>,
    packet_sender: Arc<Sender>,
}

impl Handler {
    pub fn new(packet_receiver: Arc<Receiver>, packet_sender: Arc<Sender>) -> Self {
        Handler{packet_receiver, packet_sender}
    }

    pub async fn handle(&self, mut stream: WebSocketStream<MaybeTlsStream<TcpStream>>, session: Arc<Session>, packet_receiver: mpsc::Receiver<Message>) -> Result<()> {
        let client_hello_composer = composer::ClientHello {};
        let auth_ticket_composer = composer::AuthTicket {
            sso_ticket: session.ticket.as_str(),
        };

        stream.send(client_hello_composer.compose()).await?;
        stream.send(auth_ticket_composer.compose()).await?;

        let (writer, reader) = stream.split();

        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        // Clone data for the packet receiver
        let packet_receiver_clone = self.packet_receiver.clone();
        let receiver_session = session.clone();

        let read_handle = tokio::spawn(async move {
            let resp = packet_receiver_clone.receive(reader, receiver_session, shutdown_tx).await;

            if let Err(err) = resp {
                error!("Error while receiving msg: {:?}", err);
            }
        });

        // Clone data for the packet sender
        let packet_sender_clone = self.packet_sender.clone();
        let sender_session = session.clone();

        let sender_handle = tokio::spawn(async move {
            packet_sender_clone.send(writer, sender_session, packet_receiver, shutdown_rx).await;
        });

        let _ = tokio::try_join!(sender_handle, read_handle);

        Ok(())
    }
}