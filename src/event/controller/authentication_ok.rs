use tokio::sync::broadcast;
use crate::event::ControllerEvent;

pub struct AuthenticationOkHandler {
    pub(crate) tx: broadcast::Sender<ControllerEvent>
}

impl AuthenticationOkHandler {
    pub fn handle(&self,) -> anyhow::Result<()> {
        self.tx.send(ControllerEvent::AuthenticationOk).unwrap();

        Ok(())
    }
}