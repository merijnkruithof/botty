// Event.rs is responsible for handling packets that are received by the server. This has nothing
// to do with client -> server messages.

use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

use crate::composer::{self, Composable};

const PONG: u16 = 3928;

pub async fn handle(header: u16, tx: &Sender<Message>) {
    match header {
        PONG => tx
            .send(composer::Pong {}.compose())
            .await
            .expect("unable to send pong composer to the server"),
        _ => {
            println!("No handler found for packet header {}", header);
        }
    }
}
