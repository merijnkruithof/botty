use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};
use dashmap::{DashMap, DashSet};
use tokio::sync::{broadcast, watch};
use tracing::{debug, error, info};
use crate::connection::session::Session;
use crate::event::ControllerEvent;
use crate::room::{Room, User};

use anyhow::Result;
use crate::client::state::{BotState, GlobalState};
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;

pub struct Manager {
    rooms: Arc<DashMap<u32, Arc<Room>>>,
    bots: Arc<DashMap<String, Arc<User>>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            rooms: Arc::new(DashMap::new()),
            bots: Arc::new(DashMap::new()),
        }
    }

    pub async fn listen(&self,
                        global_state: Arc<GlobalState>,
                        bot_state: Arc<BotState>,
                        event_handler_rx: broadcast::Receiver<ControllerEvent>,
                        mut kill_rx: watch::Receiver<bool>
    ) -> Result<()> {
        info!("Started room listener");

        let mut subscriber = event_handler_rx.resubscribe();
        loop {
            tokio::select! {
                Ok(data) = subscriber.recv() => self.handle_controller_event(global_state.clone(), bot_state.clone(), data),
                Ok(_) = kill_rx.changed() => {
                    if *kill_rx.borrow() {
                        break;
                    }
                }
            }
        }

        debug!("Cleaning up room listener");

        let room_id = bot_state.current_room.load(Ordering::Relaxed);
        if room_id > 0 {
            debug!("Bot {} is currently in room with id {}, removing...", &bot_state.session.ticket, &room_id);
            if let Some((ticket, user)) = self.bots.remove(&bot_state.session.ticket) {
                debug!("Removed bot {} from room user list", &ticket);

                let room = self.rooms.get(&room_id).unwrap();
                if let Some(_) = room.users.remove(&user) {
                    debug!("Bot with sso ticket {} and user id id {} removed from the room", &ticket, &user.user_id)
                }
            }
        }



        info!("Stopped room listener");

        Ok(())
    }

    fn handle_controller_event(&self, global_state: Arc<GlobalState>, bot_state: Arc<BotState>, event: ControllerEvent) {
        let room_map = self.rooms.clone();
        let bots_map = self.bots.clone();
        let user_manager = global_state.user_manager.clone();

        tokio::spawn(async move {
            match event {
                ControllerEvent::RoomLoaded { data } => {
                    let room: Arc<Room>;
                    if !room_map.contains_key(&data.room_id) {
                        // Create a new room instance
                        room = Arc::new(Room{
                            room_id: AtomicU32::new(data.room_id.clone()),
                            users: DashSet::new()
                        });

                        room_map.insert(data.room_id.clone(), room);

                        debug!("Added room with id {} to the room manager", &data.room_id);
                    } else {
                        room = room_map.get(&data.room_id).unwrap().clone();
                    }

                    bot_state.current_room.store(data.room_id.clone(), Ordering::Relaxed);

                    debug!("Loaded room with id {} for bot {}", &data.room_id, &bot_state.session.ticket);
                },

                ControllerEvent::RoomModel { data } => {
                    let _ = bot_state.packet_tx.send(composer::RequestRoomHeightmap{}.compose()).await;
                    debug!("[C->S] Sent RequestRoomHeightmap");

                    let _ = bot_state.packet_tx.send(composer::RequestRoomData { room_id: data.room_id.clone() }.compose()).await;
                    debug!("[C->S] Sent RequestRoomData for room id {}", &data.room_id.clone());
                },

                ControllerEvent::RoomUsers { data } => {
                    let room_id = bot_state.current_room.load(Ordering::Relaxed);

                    let current_user = user_manager.get_user(bot_state.session.ticket.clone()).unwrap();

                    debug!("Requesting room users of room with id {}", &room_id);
                    let room = room_map.get(&room_id).unwrap();

                    for entry in &data.users {
                        let user = User{
                            user_id: entry.user_id.clone(),
                            username: entry.username.clone(),
                            figure: entry.figure.clone(),
                            room_unit_id: entry.room_unit_id.clone(),
                            x: entry.x.clone(),
                            y: entry.y.clone(),
                            z: entry.z.clone(),
                            direction: entry.direction.clone()
                        };

                        if current_user.user_id == user.user_id {
                            bots_map.insert(bot_state.session.ticket.clone(), Arc::new(user.clone()));
                            debug!("Current session found in room, adding auth ticket {} to room {}", bot_state.session.ticket.clone(), &room_id);
                        }

                        room.users.insert(user);
                    }
                },

                _ => {}
            }
        });
    }
}