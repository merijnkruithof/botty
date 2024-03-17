use std::sync::Arc;

use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use axum::response::IntoResponse;

use crate::{composer::{self, Composable}, connection, session};

#[derive(Serialize)]
pub struct AvailableBots {
    n: usize,
}

#[derive(Deserialize)]
pub struct AvailableBotsRequest {
    hotel: String,
}

pub async fn available(
    connection_service: Extension<Arc<connection::Service>>,
    Json(payload): Json<AvailableBotsRequest>
) -> Result<impl IntoResponse, impl IntoResponse> {
    match connection_service.get_handler(payload.hotel) {
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
    connection_service: Extension<Arc<connection::Service>>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    let handler = connection_service.get_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let handler = handler.unwrap();

    let session_service_clone = handler.get_session_service();
    let message_clone = payload.message.clone();

    tokio::spawn(async move {
        session_service_clone
            .broadcast(composer::RoomUserTalk { msg: message_clone }.compose())
            .await;
    });

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct BroadcastEnterRoom {
    hotel: String,
    room_id: u32,
}

pub async fn broadcast_enter_room(
    connection_service: Extension<Arc<connection::Service>>,
    Json(payload): Json<BroadcastEnterRoom>,
) -> StatusCode {
    let handler = connection_service.get_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let handler = handler.unwrap();

    let session_service_clone = handler.get_session_service();
    let room_id = payload.room_id.clone();

    tokio::spawn(async move {
        session_service_clone
            .broadcast(composer::RequestRoomLoad { room_id }.compose())
            .await;

        session_service_clone
            .broadcast(composer::RequestRoomHeightmap {}.compose())
            .await;

        session_service_clone
            .broadcast(composer::FloorPlanEditorRequestDoorSettings {}.compose())
            .await;

        session_service_clone
            .broadcast(composer::FloorPlanEditorRequestBlockedTiles {}.compose())
            .await;

        session_service_clone
            .broadcast(composer::RequestRoomData { room_id }.compose())
            .await;
    });

    StatusCode::OK
}
