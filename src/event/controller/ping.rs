use anyhow::Result;
use tokio::sync::broadcast;

use crate::event::handler::ControllerEvent;

pub struct PingEvent { }

pub struct PingHandler {
    pub(crate) tx: broadcast::Sender<ControllerEvent>
}

impl PingHandler {
    pub fn handle(&self) -> Result<()> {
        self.tx.send(ControllerEvent::Ping).unwrap();

        Ok(())
    }
}