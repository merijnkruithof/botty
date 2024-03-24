use std::sync::Arc;

use anyhow::{anyhow, Result};
use crate::client::session;
use crate::client::session::Session;
use crate::communication::incoming::controller::handler::Handler;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::communication::packet::Reader;

pub struct PingHandler {
    session_service: Arc<session::Service>
}

impl Handler for PingHandler {
    fn new(session_service: Arc<session::Service>) -> Self {
        PingHandler{ session_service }
    }

    async fn handle(&self, session: Arc<Session>, reader: Reader) -> Result<()> {
        let composer = composer::Pong{}.compose();

        return session
            .packet_tx
            .send(composer)
            .await
            .or_else(|err| Err(anyhow!("Unable to send pong composer to the server")));
    }
}