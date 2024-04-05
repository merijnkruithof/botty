use crate::room::User;

pub struct Room {
    pub room_id: u32,
    pub users: Vec<User>
}