use std::sync::Arc;

use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use axum::response::IntoResponse;
use tracing::error;
use crate::api::actions::room;

use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::retro;

#[derive(Serialize)]
pub struct AvailableBots {
    n: usize,
}

#[derive(Deserialize)]
pub struct AvailableBotsRequest {
    hotel: String,
}

pub async fn available(
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<AvailableBotsRequest>
) -> Result<impl IntoResponse, impl IntoResponse> {
    match connection_service.get_hotel_connection_handler(payload.hotel) {
        Ok(handler) => {
            Ok((StatusCode::OK, Json(AvailableBots { n: handler.get_session_service().online_bots() })))
        },

        Err(_err) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

#[derive(Deserialize)]
pub struct BroadcastMessage {
    hotel: String,
    message: String,
}

pub async fn broadcast_message(
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let handler = handler.unwrap();

    let session_service = handler.get_session_service();
    let message_clone = payload.message.clone();

    tokio::spawn(async move {
        session_service.broadcast(composer::RoomUserTalk { msg: message_clone }.compose()).await;
    });

    StatusCode::OK
}

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
    x: u32,
    y: u32,
}

pub async fn broadcast_walk(connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<BroadcastWalk>) -> StatusCode {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::NOT_FOUND;
    }

    let handler = handler.unwrap();
    let session_service = handler.get_session_service();

    let x = payload.x.clone();
    let y = payload.y.clone();

    // tokio::spawn(async move {
    //     let msg = composer::WalkInRoom{ x,y}.compose();
    //
    //     session_service.broadcast(msg).await;
    // });

    StatusCode::OK
}