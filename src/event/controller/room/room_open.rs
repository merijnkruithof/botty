use tokio::sync::broadcast;
use crate::event::ControllerEvent;

pub struct RoomOpenHandler {
    pub tx: broadcast::Sender<ControllerEvent>
}

impl RoomOpenHandler {
    pub fn handle(&self) -> anyhow::Result<()> {
        self.tx.send(ControllerEvent::RoomOpen).unwrap();

        Ok(())
    }
}