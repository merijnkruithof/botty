use anyhow::{anyhow, Result};
use tokio::sync::broadcast;

use crate::communication::packet::Reader;
use crate::event::controller::room::{RoomLoadedEvent, RoomModelEvent, RoomUsersEvent, RoomUserStatusEvent};
use crate::event::controller::user_info::UserInfoEvent;
use crate::event::controller_factory::Factory;

#[derive(Debug, Clone)]
pub enum ControllerEvent {
    // User
    Ping,
    AuthenticationOk,
    UserInfo { data: UserInfoEvent },

    // Rooms
    RoomLoaded { data: RoomLoadedEvent },
    RoomModel { data: RoomModelEvent },
    RoomUserStatus { data: RoomUserStatusEvent },
    RoomUsers { data: RoomUsersEvent },
    RoomOpen
}

pub struct Handler {
    controller_factory: Factory,
    pub tx: broadcast::Sender<ControllerEvent>
}

impl Handler {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        let controller_factory = Factory{};

        Handler { controller_factory, tx }
    }

    pub fn handle(&self, packet: Vec<u8>) -> Result<()> {
        let mut reader = Reader::new(packet);

        if let Some(header) = reader.read_uint16() {
            if let Ok(controller) = self.controller_factory.make_controller(header, self.tx.clone()) {
                let _ = controller.handle(reader);
            }

            return Ok(());
        }

        Err(anyhow!("Packet not found"))
    }
}