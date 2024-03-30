use std::sync::Arc;
use std::sync::atomic::Ordering;
use tracing::{debug, error, info};
use tracing::field::debug;
use crate::client::room::{User, UserEvent};
use crate::client::session;
use crate::client::session::{Service, Session};
use crate::communication::incoming::controller;
use crate::communication::packet::Reader;

pub struct RoomUserStatusHandler {
    session_service: Arc<Service>
}

impl controller::handler::Handler for RoomUserStatusHandler {
    fn new(session_service: Arc<Service>) -> Self {
        RoomUserStatusHandler { session_service }
    }

    async fn handle(&self, session: Arc<Session>, mut reader: Reader) -> anyhow::Result<()> {
        let read_lock = session.room.read().await;
        if (read_lock.is_none()) {
            error!("Cannot execute RoomUserStatusHandler; bot is not in a room");
            return Ok(());
        }

        let mut total_units = reader.read_uint32().unwrap() as usize;

        let room = read_lock.as_ref().unwrap();
        while total_units > 0 {
            let room_unit_id = reader.read_uint32().unwrap();
            let x = reader.read_uint32().unwrap();
            let y = reader.read_uint32().unwrap();
            let z = reader.read_string().unwrap();
            let head_direction = reader.read_uint32().unwrap();
            let direction = reader.read_uint32().unwrap();

            let actions = reader.read_string().unwrap();

            total_units -= 1;

            if (!room.contains_room_unit_id(&room_unit_id)) {
                continue;
            }

            let user_arc = room.get_user_mut_by_room_unit_id(&room_unit_id).unwrap();
            let mut user = user_arc.write().await;

            let is_currently_walking = actions.contains("mv");
            let user_wasnt_walking_before = !user.is_walking;

            // If the user wasn't walking before, but now is, we'll have to reset the
            // times_walked_to_achieve_x_y counter, because the user is walking towards a new path.
            if user_wasnt_walking_before && is_currently_walking {
                // Dispatch "CurrentlyMoving" event
            } else {
                user.dispatch(UserEvent::UserMoved {
                    x: user.x.clone(),
                    y: user.y.clone()
                });
            }

            user.x = x;
            user.y = y;
            user.is_walking = is_currently_walking;
        }

        Ok(())
    }
}