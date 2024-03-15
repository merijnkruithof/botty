use std::sync::Arc;

use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    composer::{self, Composable},
    session,
};

#[derive(Serialize)]
pub struct AvailableBots {
    n: usize,
}

pub async fn available(
    session_service: Extension<Arc<session::Service>>,
) -> (StatusCode, Json<AvailableBots>) {
    let n = session_service.online_bots().await;

    (StatusCode::OK, Json(AvailableBots { n }))
}

#[derive(Deserialize)]
pub struct BroadcastMessage {
    message: String,
}

pub async fn broadcast_message(
    session_service: Extension<Arc<session::Service>>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    let session_service_clone = session_service.clone();
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
    room_id: u32,
}

pub async fn broadcast_enter_room(
    session_service: Extension<Arc<session::Service>>,
    Json(payload): Json<BroadcastEnterRoom>,
) -> StatusCode {
    let session_service_clone = session_service.clone();
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
