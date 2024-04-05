use anyhow::anyhow;
use tokio::sync::broadcast;
use crate::event::controller::authentication_ok;
use crate::event::controller::authentication_ok::AuthenticationOkHandler;

use crate::event::controller::handler::Controller;
use crate::event::controller::handler::Controller::{AuthenticationOk, UserData};
use crate::event::controller::ping::PingHandler;
use crate::event::controller::user_info::UserInfoHandler;
use crate::event::handler::ControllerEvent;
use crate::event::messages::Messages;

pub struct Factory { }

impl Factory {
    pub fn new() -> Self {
        Factory {}
    }

    pub fn make_controller(&self, header: u16, tx: broadcast::Sender<ControllerEvent>) -> anyhow::Result<Controller> {
        return match(header) {
            Messages::PING => Ok(Controller::Ping(PingHandler{ tx })),

            Messages::AUTHENTICATION_OK => Ok(AuthenticationOk(AuthenticationOkHandler { tx })),
            Messages::USER_DATA => Ok(UserData(UserInfoHandler::new(tx))),
            // Messages::ROOM_MODEL => Ok(Controller::RoomModel(controller::room_model::RoomModelHandler::new())),
            // Messages::ROOM_LOAD => Ok(Controller::RoomLoad(controller::room_load::RoomLoadedHandler::new(hotel_manager))),
            // Messages::ROOM_USER_STATUS => Ok(Controller::RoomUserStatus(controller::room_user_status::RoomUserStatusHandler::new(hotel_manager))),
            // Messages::ROOM_USERS => Ok(Controller::RoomUsers(controller::room_users::RoomUsersHandler::new(hotel_manager))),

            _ => Err(anyhow!("No controller found for header {}", header))
        }
    }
}