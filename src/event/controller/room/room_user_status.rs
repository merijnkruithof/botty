use tokio::sync::broadcast;
use crate::communication::packet::Reader;
use crate::event::ControllerEvent;
use crate::event::ControllerEvent::RoomUserStatus;
use crate::event::parser::room::room_user_status_parser;

pub struct RoomUserStatusHandler {
    pub tx: broadcast::Sender<ControllerEvent>
}

#[derive(Clone, Debug)]
pub struct RoomUserStatusEvent {
    pub total_units: usize,
    pub room_units: Vec<RoomUnit>
}

#[derive(Clone, Debug)]
pub struct RoomUnit {
    pub room_unit_id: u32,
    pub x: u32,
    pub y: u32,
    pub z: String,
    pub head_direction: u32,
    pub direction: u32,
    pub actions: String,
}

impl RoomUserStatusHandler {
    pub fn handle(&self, reader: Reader) -> anyhow::Result<()> {
        let event = room_user_status_parser::parse(reader);

        self.tx.send(RoomUserStatus { data: event }).unwrap();

        Ok(())
    }
}