use std::sync::Arc;

use anyhow::{anyhow, Result};
use crate::client::session::Session;
use crate::communication::incoming::controller::handler::Handler;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::communication::packet::Reader;

pub struct PingHandler {}

impl Handler for PingHandler {
    async fn handle(&self, session: Arc<Session>, reader: Reader) -> Result<()> {
        let composer = composer::Pong{}.compose();

        return session
            .packet_tx
            .send(composer)
            .await
            .or_else(|err| Err(anyhow!("Unable to send pong composer to the server")));
    }
}