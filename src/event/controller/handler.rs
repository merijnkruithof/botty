use anyhow::Result;

use crate::communication::packet::Reader;
use crate::event::controller::ping::PingHandler;
use crate::event::controller::room::{RoomLoadedHandler, RoomModelHandler, RoomOpenHandler, RoomUsersHandler, RoomUserStatusHandler};
use crate::event::controller::user::AuthenticationOkHandler;
use crate::event::controller::user_info::UserInfoHandler;

pub enum Controller {
    Ping(PingHandler),
    AuthenticationOk(AuthenticationOkHandler),
    UserData(UserInfoHandler),
    RoomModel(RoomModelHandler),
    RoomOpen(RoomOpenHandler),
    RoomLoad(RoomLoadedHandler),
    RoomUserStatus(RoomUserStatusHandler),
    RoomUsers(RoomUsersHandler)
}

impl Controller {
    pub fn handle(&self, reader: Reader) -> Result<()> {
        let _event = match self {
            Controller::Ping(handler) => handler.handle(),
            Controller::AuthenticationOk(handler) => handler.handle(),
            Controller::UserData(handler) => handler.handle(reader),
            Controller::RoomModel(handler) => handler.handle(reader),
            Controller::RoomLoad(handler) => handler.handle(reader),
            Controller::RoomUserStatus(handler) => handler.handle(reader),
            Controller::RoomUsers(handler) => handler.handle(reader),
            Controller::RoomOpen(handler) => handler.handle(),
        };

        Ok(())
    }
}