

use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};

pub struct Sender { }

impl Sender {
    pub async fn send(
        &mut self,
        mut rx: mpsc::Receiver<Message>,
        mut writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        mut shutdown_rx: watch::Receiver<bool>
    ) {
        loop {
            tokio::select! {
                data = rx.recv() => {
                    if let Some(msg) = data {
                        if let Err(err) = writer.send(msg).await {
                            error!("An error occurred while trying to send data to the websocket: {:?}", err);
                            continue;
                        }
                    }
                },

                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        info!("Sender closed due to shutdown signal");
                        break;
                    }
                }
            }
        }
    }
}