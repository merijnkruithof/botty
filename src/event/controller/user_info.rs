use tokio::sync::broadcast;
use crate::communication::packet::Reader;
use crate::event::ControllerEvent;
use crate::event::parser::user::user_data_parser;

use anyhow::Result;

#[derive(Clone, Debug)]
pub struct UserInfoEvent {
    pub user_id: u32,
    pub username: String,
    pub figure: String,
    pub gender: String,
    pub motto: String,
    pub real_name: String,
    pub direct_mail: bool,
    pub respects_received: u32,
    pub respects_remaining: u32,
    pub respects_pet_remaining: u32,
    pub stream_publishing_allowed: bool,
    pub last_access_date: String,
    pub can_change_name: bool,
    pub safety_locked: bool
}

pub struct UserInfoHandler {
    tx: broadcast::Sender<ControllerEvent>
}

impl UserInfoHandler {
    pub fn new(tx: broadcast::Sender<ControllerEvent>) -> Self {
        UserInfoHandler { tx }
    }

    pub fn handle(&self, reader: Reader) -> Result<()> {
        let data = user_data_parser::parse(reader);

        self.tx.send(ControllerEvent::UserInfo { data }).unwrap();

        Ok(())
    }
}