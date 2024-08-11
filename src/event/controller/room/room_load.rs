use tokio::sync::broadcast;
use crate::communication::packet::Reader;
use crate::event::ControllerEvent;
use crate::event::parser::room::room_load_parser;

#[derive(Clone, Debug)]
pub struct RoomLoadedEvent {
    pub room_id: u32,
}

pub struct RoomLoadedHandler {
    tx: broadcast::Sender<ControllerEvent>
}

impl RoomLoadedHandler {
    pub fn new(tx: broadcast::Sender<ControllerEvent>) -> Self {
        RoomLoadedHandler{ tx }
    }

    pub fn handle(&self, reader: Reader) -> anyhow::Result<()> {
        let room_loaded_event = room_load_parser::parse(reader);

        // Dispatch RoomLoadedEvent
        self.tx.send(ControllerEvent::RoomLoaded { data: room_loaded_event })?;

        Ok(())
    }
}