use tokio::sync::broadcast;
use crate::communication::packet::Reader;
use crate::event::ControllerEvent;
use crate::event::parser::room::room_users_parser;

#[derive(Clone, Debug)]
pub struct RoomUsersEvent {
    pub(crate) total_users: u32,
    pub(crate) users: Vec<RoomUser>
}

#[derive(Clone, Debug)]
pub struct RoomUser {
    pub user_id: u32,
    pub username: String,
    pub custom: String,
    pub figure: String,
    pub room_unit_id: u32,
    pub x: u32,
    pub y: u32,
    pub z: String,
    pub direction: u32,
    pub user_type: u32
}

pub struct RoomUsersHandler {
    pub tx: broadcast::Sender<ControllerEvent>
}

impl RoomUsersHandler {
    pub fn handle(&self, reader: Reader) -> anyhow::Result<()> {
        let event = room_users_parser::parse(reader);

        self.tx.send(ControllerEvent::RoomUsers { data: event }).unwrap();

        Ok(())
    }
}