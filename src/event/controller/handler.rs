use anyhow::Result;

use crate::communication::packet::Reader;
use crate::event::controller::authentication_ok::AuthenticationOkHandler;
use crate::event::controller::ping::PingHandler;
use crate::event::controller::user_info::UserInfoHandler;

pub enum Controller {
    Ping(PingHandler),
    AuthenticationOk(AuthenticationOkHandler),
    UserData(UserInfoHandler)
    // RoomModel(controller::room_model::RoomModelHandler),
    // RoomLoad(controller::room_load::RoomLoadedHandler),
    // RoomUserStatus(controller::room_user_status::RoomUserStatusHandler),
    // RoomUsers(controller::room_users::RoomUsersHandler)
}

impl Controller {
    pub fn handle(&self, reader: Reader) -> Result<()> {
        let event = match self {
            Controller::Ping(handler) => handler.handle(),
            Controller::AuthenticationOk(handler) => handler.handle(),
            Controller::UserData(handler) => handler.handle(reader),
            // Controller::RoomModel(handler) => handler.handle(session, reader).await,
            // Controller::RoomLoad(handler) => handler.handle(session, reader).await,
            // Controller::RoomUserStatus(handler) => handler.handle(session, reader).await,
            // Controller::RoomUsers(handler) => handler.handle(session, reader).await
        };

        // Dispatch the event to the whole application

        Ok(())
    }
}