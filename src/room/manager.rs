use dashmap::DashMap;
use tracing::error;
use crate::room::Room;

pub struct Manager {
    rooms: DashMap<u32, Room>
}

impl Manager {
    pub fn new() -> Self {
        Manager { rooms: DashMap::new() }
    }

    pub fn bot_is_in_room(&self, room_id: &u32, user_id: &u32) -> bool {
        if let Some(room) = self.rooms.get(room_id) {
            if room.contains_user_id(user_id) { true } else { false }
        } else {
            error!("Tried to get room with id {}, but it does not exist", room_id);
            false
        }
    }
}