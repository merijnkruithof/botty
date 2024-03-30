use std::sync::Arc;
use std::sync::atomic::AtomicIsize;
use tracing::{info, warn};
use crate::client::{room, session};
use crate::client::session::{Service, Session};
use crate::communication::incoming::controller;
use crate::communication::packet::Reader;

pub struct RoomUsersHandler {
    session_service: Arc<session::Service>
}

impl controller::handler::Handler for RoomUsersHandler {
    fn new(session_service: Arc<Service>) -> Self {
        RoomUsersHandler{ session_service }
    }

    async fn handle(&self, session: Arc<Session>, mut reader: Reader) -> anyhow::Result<()> {
        let read_lock = session.room.read().await;
        if read_lock.is_none() {
            warn!("User is not in a room");
            return Ok(())
        }

        let total_users = reader.read_uint32().unwrap();
        let mut users: Vec<room::User> = Vec::new();

        let mut i = 0;
        while i < total_users {
            let user_id = reader.read_uint32().unwrap();
            let username = reader.read_string().unwrap();
            let _ = reader.read_string().unwrap(); // custom
            let figure = reader.read_string().unwrap();
            let room_unit_id = reader.read_uint32().unwrap();
            let x = reader.read_uint32().unwrap();
            let y = reader.read_uint32().unwrap();
            let _ = reader.read_string().unwrap(); // z
            let _ = reader.read_uint32().unwrap(); // direction
            let t_type = reader.read_uint32().unwrap();

            if t_type == 1 {
                let web_id = user_id;
                // add user_type = RoomObjectType.USER
                let _ = "User";
                let sex = reader.read_string().unwrap(); // sex
                let _ = reader.read_uint32().unwrap(); // group_id
                let _ = reader.read_uint32().unwrap(); // group_status
                let _ = reader.read_string().unwrap(); // group_name
                let _ = reader.read_string().unwrap(); // swim_figure
                let _ = reader.read_uint32().unwrap(); // activity_points
                let _ = reader.read_bool().unwrap(); // is_moderator

                users.push(room::User::new(
                    user_id.clone(),
                    room_unit_id.clone(),
                    username.clone(),
                    figure.clone(),
                    sex.clone(),
                    x.clone(),
                    y.clone()
                ));
            } else if t_type == 2 {
                // we're not doing anything with pets right now, but we'll parse it anyway as it's
                // required lmao
                let _ = reader.read_uint32().unwrap(); // sub_type
                let _ = reader.read_uint32().unwrap(); // owner_id
                let _ = reader.read_string().unwrap(); // owner_name
                let _ = reader.read_uint32().unwrap(); // rarity_level
                let _ = reader.read_bool().unwrap(); // has_saddle
                let _ = reader.read_bool().unwrap(); // is_riding
                let _ = reader.read_bool().unwrap(); // can_breed
                let _ = reader.read_bool().unwrap(); // can_harvest
                let _ = reader.read_bool().unwrap(); // can_revive
                let _ = reader.read_bool().unwrap(); // has_breeding_permission
                let _ = reader.read_uint32().unwrap(); // pet_level
                let _ = reader.read_string().unwrap(); // pet_posture
            } else if t_type == 3 {
                // bots. nothing to read here.
            } else if t_type == 4 {
                // rentable bot
                let _ = reader.read_string().unwrap(); // sex
                let _ = reader.read_uint32().unwrap(); // owner_id
                let _ = reader.read_string().unwrap(); // owner_name

                let total_skills = reader.read_uint32().unwrap();
                let mut j = 0;
                while j < total_skills {
                    let _ = reader.read_uint16(); // skill
                    j += 1;
                }
            }

            i += 1;
        }

        // update room users
        let read_lock = read_lock.as_ref().unwrap();

        read_lock.add_users(users).await;

        Ok(())
    }
}