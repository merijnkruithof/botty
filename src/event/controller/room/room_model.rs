use tokio::sync::broadcast;
use crate::communication::packet::Reader;
use crate::event::ControllerEvent;
use crate::event::ControllerEvent::RoomModel;
use crate::event::parser::room::room_model_parser;

pub struct RoomModelHandler {
    pub(crate) tx: broadcast::Sender<ControllerEvent>
}

#[derive(Clone, Debug)]
pub struct RoomModelEvent {
    pub model: String,
    pub room_id: u32
}

impl RoomModelHandler {
    pub fn handle(&self, reader: Reader) -> anyhow::Result<()> {
        let event = room_model_parser::parse(reader);

        self.tx.send(RoomModel { data: event }).unwrap();

        Ok(())
    }
}