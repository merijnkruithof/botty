// Event.rs is responsible for handling packets that are received by the server. This has nothing
// to do with client -> server messages.

use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

use crate::composer::{self, Composable};

const PING: u16 = 3928;

pub async fn handle(header: u16, tx: &Sender<Message>) {
    match header {
        PING => tx
            .send(composer::Pong {}.compose())
            .await
            .expect("unable to send pong composer to the server"),
        _ => {
            // do nothing
        }
    }
}
