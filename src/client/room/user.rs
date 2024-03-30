use std::sync::atomic::{AtomicI32, AtomicIsize};
use anyhow::{anyhow, Error};
use log::{debug, error};
use tokio::sync::broadcast;

pub struct User {
    pub(crate) user_id: u32,
    pub(crate) room_unit_id: u32,
    pub(crate) username: String,
    pub(crate) figure: String,
    pub(crate) sex: String,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) is_walking: bool,

    tx: broadcast::Sender<UserEvent>,
}

impl User {
    pub fn new(user_id: u32, room_unit_id: u32, username: String, figure: String, sex: String, x: u32, y: u32) -> Self {
        // Set default values
        let is_walking = false;

        // Allow application code to listen to certain events.
        let (tx, _) = broadcast::channel(10); // TODO: determine ch capacity.

        User {
            user_id,
            room_unit_id,
            username,
            figure,
            sex,
            x,
            y,
            is_walking,
            tx
        }
    }

    pub fn dispatch(&self, msg: UserEvent) {
        if let Err(err) = self.tx.send(msg) {
            error!("Unable to dispatch event: {:?}", err);
        }
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<UserEvent> {
        self.tx.subscribe()
    }
}

#[derive(Clone, Debug)]
pub enum UserEvent {
    UserMoved { x: u32, y: u32 }
}