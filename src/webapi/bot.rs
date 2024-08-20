use std::sync::Arc;


use anyhow::Result;
use axum::{Extension, Json};
use axum::response::IntoResponse;

use http::StatusCode;


use serde::{Deserialize, Serialize};


use tracing::{error};

use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::core::taskmgr::task;
use crate::core::taskmgr::task::KillableTask;

use crate::retro;
use crate::user::User;
use crate::webapi::actions::room;



#[derive(Deserialize)]
pub struct BroadcastEnterRoom {
    hotel: String,
    room_id: u32,
}

pub async fn broadcast_enter_room(
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<BroadcastEnterRoom>,
) -> StatusCode {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let handler = handler.unwrap();

    let session_service = handler.get_session_service();
    let room_id = payload.room_id.clone();

    tokio::spawn(async move {
        if let Err(err) = room::broadcast_enter(room_id, session_service).await {
            error!("Unable to enter room: {:?}", err);
        }
    });

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct BroadcastWalk {
    hotel: String,
    following_user_id: u32,
}

struct Followed {
    target_x: u32,
    target_y: u32,
}

#[derive(Deserialize)]
pub struct BroadcastAbuseCfh {
    hotel: String,
    message: String,
    user_id: Option<i32>,
    room_id: u32,
}

pub async fn broadcast_cfh_abuse(retro_manager: Extension<Arc<retro::Manager>>, Json(payload): Json<BroadcastAbuseCfh>) -> StatusCode {
    let handler = retro_manager.get_hotel_connection_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::NOT_FOUND;
    }

    let handler = handler.unwrap();

    handler.get_session_service().broadcast(composer::ReportComposer{
        message: payload.message,
        topic: 35,
        room_id: payload.room_id,
        user_id: if payload.user_id.is_none() { -1 } else { payload.user_id.unwrap() },
    }.compose()).await;

    StatusCode::OK
}
