use std::sync::{Arc, RwLock};
use std::sync::atomic::Ordering;
use dashmap::DashMap;
use tokio::sync::{broadcast, watch};
use tracing::{debug, error, info};
use crate::connection::session::Session;
use crate::event::ControllerEvent;
use crate::room::{Room, User};

use anyhow::Result;
use crate::client::state::BotState;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;

pub struct Manager {
    rooms: Arc<DashMap<u32, Arc<RwLock<Room>>>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            rooms: Arc::new(DashMap::new()),
        }
    }

    pub async fn listen(&self, bot_state: Arc<BotState>, event_handler_rx: broadcast::Receiver<ControllerEvent>, mut kill_rx: watch::Receiver<bool>) -> Result<()> {
        info!("Started room listener");

        let mut subscriber = event_handler_rx.resubscribe();
        loop {
            tokio::select! {
                Ok(data) = subscriber.recv() => self.handle_controller_event(bot_state.clone(), data),
                Ok(_) = kill_rx.changed() => {
                    if *kill_rx.borrow() {
                        break;
                    }
                }
            }
        }

        info!("Stopped room listener");

        Ok(())
    }
    fn handle_controller_event(&self, bot_state: Arc<BotState>, event: ControllerEvent) {
        let room_map = self.rooms.clone();

        tokio::spawn(async move {
            match event {
                ControllerEvent::RoomModel { data } => {
                    bot_state.current_room.store(data.room_id.clone(), Ordering::Relaxed);

                    // create a new room if it doesn't exist yet.
                    if !room_map.contains_key(&data.room_id) {
                        let room = Arc::new(RwLock::new(Room{
                            room_id: data.room_id.clone(),
                            users: vec![]
                        }));

                        room_map.insert(data.room_id.clone(), room);
                    }

                    let _ = bot_state.packet_tx.send(composer::RequestRoomHeightmap{}.compose()).await;
                    debug!("[C->S] Sent RequestRoomHeightmap");

                    let _ = bot_state.packet_tx.send(composer::RequestRoomData { room_id: data.room_id.clone() }.compose()).await;
                    debug!("[C->S] Sent RequestRoomData for room id {}", &data.room_id.clone());
                },

                ControllerEvent::RoomUsers { data } => {
                    let room_id = bot_state.current_room.load(Ordering::Relaxed);
                    let room = room_map.get(&room_id).unwrap();
                    let mut write_lock = room.write().unwrap();

                    let users = data.users
                        .iter()
                        .map(|entry| {
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

                            debug!("Adding user {:?} to room", &user);

                            user
                        })
                        .collect();

                    write_lock.users = users;
                },

                _ => {}
            }
        });
    }
}