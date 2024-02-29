use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

pub struct Session {
    pub ticket: String,
    pub tx: Sender<Message>,
}
