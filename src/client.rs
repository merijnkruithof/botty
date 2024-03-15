use std::sync::Arc;
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt, };
use tokio::{net::TcpStream, sync::{mpsc::Receiver, mpsc::Sender, watch}};
use tokio_tungstenite::{connect_async, tungstenite::{handshake::client::Request, http::Uri, protocol::Message, Error}, MaybeTlsStream, WebSocketStream};

use crate::{composer::{self, Composable}, event, packet};
use anyhow::{anyhow, Result};
use tracing::{error, info};
use tracing::log::trace;
use crate::session::Session;

pub async fn connect(ws_link: String, origin: String) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let uri = Uri::try_from(ws_link)?;

    let request = Request::builder()
        .uri(uri)
        .header("Origin", origin)
        .body(())
        .expect("Failed to build request");

    let (ws_stream, _) = connect_async(request).await?;

    Ok(ws_stream)
}

async fn handle_read(
    mut reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    tx: Sender<Message>,
    shutdown_tx: watch::Sender<bool>,
    mut kill_sig: watch::Receiver<bool>,
) -> Result<()> {
    let shutdown_fn = ||  {
        // consider not sending to the shutdown data as something unrecoverable. I'd rather kill
        // the whole thread
        shutdown_tx.send(true).unwrap();

        info!("Reader closed, shutdown signal sent to listeners");
    };

    loop {
        tokio::select! {
            data = reader.next() => {
                if let Some(Ok(msg)) = data {
                    let handled_client_msg = handle_client_message(msg, tx.clone()).await;

                    if let Err(err) = handled_client_msg {
                        shutdown_fn();
                        break;
                    }
                } else {
                    shutdown_tx.send(true)?;
                    break;
                }
            },

            _ = kill_sig.changed() => {
                if *kill_sig.borrow() {
                    // initiate shutdown. no need to handle shutdown_tx here; the writer listens to
                    // kill_sig as well.
                    info!("Kill sig triggered, reader is now closed.");
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn handle_client_message(msg: Message, tx: Sender<Message>) -> Result<()> {
    if msg.is_close() {
        return Err(anyhow!("Websocket connection is closed"));
    }

    let data = msg.into_data();

    // Create a new reader instance.
    let mut reader = packet::Reader::new(&data);

    if let Some(header) = reader.read_uint16() {
        event::handle(header, &tx).await;
    }

    Ok(())
}

async fn handle_write(
    mut writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut ch: Receiver<Message>,
    mut shutdown_rx: watch::Receiver<bool>,
    mut kill_sig: watch::Receiver<bool>,
) {
    loop {
        tokio::select! {
            _ = kill_sig.changed() => {
                if *kill_sig.borrow() {
                    info!("Kill sig triggered, writer is killed");
                    break;
                }
            }

            data = ch.recv() => {
                if let Some(msg) = data {
                    if let Err(err) = writer.send(msg).await {
                        error!("An error occurred while trying to send data to the websocket: {:?}", err);
                        continue;
                    }
                }
            },

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    info!("Writer closed, shutdown channel has been triggered");
                    break;
                }
            }
        }
    }
}

// Handle takes care of handling the client to server connection.
//
// It enables client -> server communication, as well as server -> client communication. It spawns
pub async fn handle(
    mut stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    rx: Receiver<Message>,
    tx: Sender<Message>,
    session: Arc<Session>,
) -> Result<()> {
    let client_hello_composer = composer::ClientHello {};
    let auth_ticket_composer = composer::AuthTicket {
        sso_ticket: session.ticket.as_str(),
    };

    stream.send(client_hello_composer.compose()).await?;
    stream.send(auth_ticket_composer.compose()).await?;

    let (writer, reader) = stream.split();

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let read_handle = tokio::spawn(handle_read(reader, tx, shutdown_tx, session.kill_sig_rx.clone()));
    let write_handle = tokio::spawn(handle_write(writer, rx, shutdown_rx.clone(), session.kill_sig_rx.clone()));

    let _ = tokio::try_join!(write_handle, read_handle);

    Ok(())
}
