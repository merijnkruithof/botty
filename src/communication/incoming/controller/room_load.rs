use std::sync::Arc;
use anyhow::anyhow;
use tracing::{debug, error};
use crate::client::{room, session};
use crate::client::session::{Service, Session};
use crate::communication::incoming::controller;
use crate::communication::packet::Reader;

pub struct RoomLoadedHandler {
    session_service: Arc<Service>
}

impl controller::handler::Handler for RoomLoadedHandler {
    fn new(session_service: Arc<Service>) -> Self {
        RoomLoadedHandler { session_service }
    }

    async fn handle(&self, session: Arc<Session>, mut reader: Reader) -> anyhow::Result<()> {
        let _ = reader.read_bool();
        match reader.read_uint32() {
            Some(room_id) => {
                // do not set the room if the user is already there. hacky solution, as there's
                // something wrong with room entering, and currently I can't be arsed.
                if let Some(current_room) = session.room.read().await.as_ref() {
                    if current_room.room_id == room_id.clone() {
                        debug!("Not changing room id, bot is already in the room");
                        return Ok(());
                    }
                }

                debug!("Changing room id of session {:} to {:}", &session.ticket, &room_id);

                session.set_room(room::Room::new(room_id)).await;
            },

            None => {
                error!("Unable to get room id from packet reader");
            }
        }

        Ok(())
    }
}