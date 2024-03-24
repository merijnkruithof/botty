use std::sync::Arc;
use anyhow::{anyhow, Result};
use crate::client;
use crate::client::session::Session;
use crate::communication::incoming::{controller};
use crate::communication::packet::Reader;

pub struct Handler {
    session_service: Arc<client::session::Service>,
    controller_factory: Arc<controller::handler::Factory>
}

impl Handler {
    pub fn new(controller_factory: Arc<controller::handler::Factory>, session_service: Arc<client::session::Service>) -> Self {
        Handler { session_service, controller_factory }
    }

    pub async fn handle(&self, session: Arc<Session>, packet: Vec<u8>) -> Result<()> {
        let mut reader = Reader::new(packet);

        if let Some(header) = reader.read_uint16() {
            if let Ok(controller) = self.controller_factory.make_controller(header, self.session_service.clone()) {
                let _ = controller.handle(session, reader).await?;
            }

            return Ok(());
        }

        Err(anyhow!("Packet not found"))
    }
}