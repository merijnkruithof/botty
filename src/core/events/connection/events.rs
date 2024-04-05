use tokio_tungstenite::tungstenite::Message;

#[derive(Clone, Debug)]
pub enum ReceiverEvent {
    MessageReceived { msg: Message },
    ReceiverClosed
}