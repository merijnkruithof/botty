use std::sync::Arc;
use anyhow::{anyhow, Result};
use crate::client::session;
use crate::client::session::Session;
use crate::communication::incoming::controller;
use crate::communication::incoming::messages::Messages;
use crate::communication::packet::Reader;

pub trait Handler {
    fn new(session_service: Arc<session::Service>) -> Self;

    async fn handle(&self, session: Arc<Session>, reader: Reader) -> Result<()>;
}

pub struct Factory { }

impl Factory {
    pub fn new() -> Self {
        Factory {}
    }

    pub fn make_controller(&self, header: u16, session_service: Arc<session::Service>) -> Result<Controller> {
        return match(header) {
            Messages::PING => Ok(Controller::Ping(controller::ping::PingHandler::new(session_service))),
            Messages::ROOM_MODEL => Ok(Controller::RoomModel(controller::room_model::RoomModelHandler::new(session_service))),
            Messages::ROOM_LOAD => Ok(Controller::RoomLoad(controller::room_load::RoomLoadedHandler::new(session_service))),
            Messages::ROOM_USER_STATUS => Ok(Controller::RoomUserStatus(controller::room_user_status::RoomUserStatusHandler::new(session_service))),
            Messages::ROOM_USERS => Ok(Controller::RoomUsers(controller::room_users::RoomUsersHandler::new(session_service))),

            _ => Err(anyhow!("No controller found for header {}", header))
        }
    }
}

pub enum Controller {
    Ping(controller::ping::PingHandler),
    RoomModel(controller::room_model::RoomModelHandler),
    RoomLoad(controller::room_load::RoomLoadedHandler),
    RoomUserStatus(controller::room_user_status::RoomUserStatusHandler),
    RoomUsers(controller::room_users::RoomUsersHandler)
}

impl Controller {
    pub async fn handle(&self, session: Arc<Session>, reader: Reader) -> Result<()> {
        match self {
            Controller::Ping(handler) => handler.handle(session, reader).await,
            Controller::RoomModel(handler) => handler.handle(session, reader).await,
            Controller::RoomLoad(handler) => handler.handle(session, reader).await,
            Controller::RoomUserStatus(handler) => handler.handle(session, reader).await,
            Controller::RoomUsers(handler) => handler.handle(session, reader).await
        }
    }
}