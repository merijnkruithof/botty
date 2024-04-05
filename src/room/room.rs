use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use log::error;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::RwLock;
use tracing::info;
use crate::room::User;

pub struct Room {
    pub(crate) room_id: u32,

    // users is DashMap<user_id, User>
    users: Arc<DashMap<u32, Arc<RwLock<User>>>>,

    // room_users is DashMap<room_unit_id, User>
    room_users: Arc<DashMap<u32, Arc<RwLock<User>>>>,
}

impl Room {
    pub fn new(room_id: u32) -> Self {
        let users = Arc::new(DashMap::new());
        let room_users = Arc::new(DashMap::new());

        let room = Room {
            room_id,
            users: users.clone(),
            room_users: room_users.clone(),
        };

        room
    }

    pub fn contains_user_id(&self, user_id: &u32) -> bool {
        return self.users.contains_key(user_id);
    }

    pub fn contains_room_unit_id(&self, room_unit_id: &u32) -> bool {
        return self.room_users.contains_key(room_unit_id);
    }

    pub fn get_user_mut_by_room_unit_id(&self, room_unit_id: &u32) -> Option<RefMut<u32, Arc<RwLock<User>>>> {
        return self.room_users.get_mut(room_unit_id);
    }

    pub fn get_user_by_user_id(&self, user_id: &u32) -> Option<Ref<u32, Arc<RwLock<User>>>> {
        return self.users.get(user_id);
    }

    pub async fn add_users(&self, users: Vec<User>) {
        for user in users {
            let user_id = user.user_id.clone();
            let room_unit_id = user.room_unit_id.clone();

            let user = Arc::new(RwLock::new(user));

            self.users.insert(user_id, user.clone());
            self.room_users.insert(room_unit_id, user.clone());
        }
    }

    pub async fn remove_room_user(&self, room_unit_id: &u32) {
        let user = self.room_users.get(room_unit_id);
        if user.is_none() {
            error!("Unable to remove user from room, room unit id {:?} does not exist", &room_unit_id);
            return;
        }

        let user = user.unwrap();
        let user = user.read().await;

        self.room_users.remove(room_unit_id);
        self.users.remove(&user.user_id);

        info!("Removed user from the room");
    }
}

