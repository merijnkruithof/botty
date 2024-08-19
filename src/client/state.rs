use std::sync::Arc;
use std::sync::atomic::AtomicU32;

use tokio::sync::{broadcast, mpsc, watch};
use tokio_tungstenite::tungstenite::Message;

use crate::connection::session;
use crate::connection::session::Session;
use crate::core::events::connection::ReceiverEvent;
use crate::{event, room, user};

pub struct GlobalState {
    pub session_service: Arc<session::Service>,
    pub room_manager: Arc<room::Manager>,
    pub user_manager: Arc<user::Manager>
}

impl GlobalState {
    pub fn new() -> Self {
        let session_service = Arc::new(session::Service::new());
        let room_manager = Arc::new(room::Manager::new());
        let user_manager = Arc::new(user::Manager::new());

        GlobalState {
            session_service,
            room_manager,
            user_manager
        }
    }
}

pub struct BotState {
    pub current_room: AtomicU32,

    // Contains the current network session.
    pub session: Arc<Session>,

    // Contains the channel that's able to send network traffic to the server.
    pub packet_tx: mpsc::Sender<Message>,

    // Contains the connection receiver. We'll use it to subscribe to events.
    pub receiver: broadcast::Receiver<ReceiverEvent>,

    // Contains the event handler
    pub event_handler: Arc<event::Handler>,
}

impl BotState {
    pub fn new(auth_ticket: String, packet_tx: mpsc::Sender<Message>, receiver: broadcast::Receiver<ReceiverEvent>) -> Self {
        let session = Arc::new(Session::new(auth_ticket, packet_tx.clone())); // packet_tx is here for legacy reasons.

        // Each client connection needs a receiver, sender, and handler.
        let rate_limit = 100;
        let event_handler = Arc::new(event::Handler::new(rate_limit));

        BotState{
            current_room: AtomicU32::new(0),
            session,
            receiver,
            packet_tx,
            event_handler
        }
    }
}
