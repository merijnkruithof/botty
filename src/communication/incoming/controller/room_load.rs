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
        // Parse logic from Nitro.
        // public parse(wrapper: IMessageDataWrapper): boolean
        // {
        //     if(!wrapper) return false;
        //
        //     this._roomEnter = wrapper.readBoolean();
        //     this._data = new RoomDataParser(wrapper);
        //     this._roomForward = wrapper.readBoolean();
        //     this._staffPick = wrapper.readBoolean();
        //     this._isGroupMember = wrapper.readBoolean();
        //     this.data.allInRoomMuted = wrapper.readBoolean();
        //     this._moderation = new RoomModerationSettings(wrapper);
        //     this.data.canMute = wrapper.readBoolean();
        //     this._chat = new RoomChatSettings(wrapper);
        //
        //     return true;
        // }

        let _ = reader.read_bool();
        match reader.read_uint32() {
            Some(room_id) => {
                debug!("Changing room id of session {:} to {:}", &session.ticket, &room_id);

                session.set_room(room::Room { room_id }).await;
            },

            None => {
                error!("Unable to get room id from packet reader");
            }
        }

        Ok(())
    }
}