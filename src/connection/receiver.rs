use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use futures_util::{SinkExt, stream::SplitSink, StreamExt, };
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, watch};
use tokio::sync;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};

use crate::connection::session::Session;
use crate::core::events::connection::ReceiverEvent;

pub struct Receiver {
    tx: broadcast::Sender<ReceiverEvent>
}

pub trait Subscriber {
    fn subscribe(&self) -> broadcast::Receiver<ReceiverEvent>;
}

impl Subscriber for Receiver {
    fn subscribe(&self) -> broadcast::Receiver<ReceiverEvent> {
        self.tx.subscribe()
    }
}

impl Receiver {
    pub fn new(capacity: usize) -> Self {
        // The receiver sends some messages to other parts of the components, such as MessageReceived,
        // and ReceiverClosed.
        let (tx, _) = broadcast::channel(capacity);

        Receiver{ tx }
    }

    async fn handle_client_message(&self, msg: Message) -> Result<()> {
        if msg.is_close() {
            return Err(anyhow!("Websocket connection is closed"));
        }

        if let Err(err) = self.tx.send(ReceiverEvent::MessageReceived { msg }) {
            return Err(anyhow!("Unable to send message to the channel: {:?}", err));
        }

        Ok(())
    }

    pub async fn receive(&self, mut reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>, cancellation: watch::Sender<bool>) {
        loop {
            match reader.next().await {
                Some(Ok(data)) => {
                    if let Err(err) = self.handle_client_message(data).await {
                        error!("Reader closed! Unable to handle client message: {:?}", err);

                        cancellation.send(true).unwrap();
                        return;
                    }
                },

                Some(Err(err)) => {
                    error!("Reader closed! Reason: {:?}", err);

                    cancellation.send(true).unwrap();
                    return;
                },

                _ => {
                    // Unknown situation. Close thread.
                    error!("Unknown situation! Reader closed.");

                    cancellation.send(true).unwrap();
                    return;
                }
            }
        }
    }
}