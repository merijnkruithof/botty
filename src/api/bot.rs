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
    session_service: Extension<Arc<Mutex<session::Service>>>,
) -> (StatusCode, Json<AvailableBots>) {
    let read_lock = session_service.lock().await;

    let n = read_lock.online_bots().await;

    (StatusCode::OK, Json(AvailableBots { n }))
}

#[derive(Deserialize)]
pub struct BroadcastMessage {
    message: String,
}

pub async fn broadcast_message(
    session_service: Extension<Arc<Mutex<session::Service>>>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    let session_service_clone = session_service.clone();
    let message_clone = payload.message.clone();

    tokio::spawn(async move {
        let read_lock = session_service_clone.lock().await;

        read_lock
            .broadcast(composer::RoomUserTalk { msg: message_clone }.compose())
            .await;
    });

    StatusCode::OK
}
