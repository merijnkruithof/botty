use std::sync::Arc;
use tracing::debug;
use controller::handler;
use handler::Handler;
use crate::client::session::{Service, Session};
use crate::communication::incoming::controller;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::communication::packet::Reader;

pub struct RoomModelHandler {
    session_service: Arc<Service>
}

impl Handler for RoomModelHandler {
    fn new(session_service: Arc<Service>) -> Self {
        RoomModelHandler{ session_service }
    }

    async fn handle(&self, session: Arc<Session>, reader: Reader) -> anyhow::Result<()> {
        // based on the next data (from Nitro): RoomMessageHandler.ts:156 - event.connection.send(new GetRoomEntryDataMessageComposer());
        let room_entry_data_message = composer::RequestRoomHeightmap{}.compose();
        session.packet_tx.send(room_entry_data_message).await.unwrap();

        Ok(())
    }
}