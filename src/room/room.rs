use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use dashmap::DashSet;
use crate::room::User;

pub struct Room {
    pub room_id: AtomicU32,
    pub users: DashSet<User>
}