use std::sync::Arc;
use axum::{Extension, Json};
use http::StatusCode;
use serde::Deserialize;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::retro;

#[derive(Deserialize)]
pub struct BroadcastMessage {
    hotel: String,
    message: String,
}

pub async fn broadcast_message(manager: Extension<Arc<retro::Manager>>, Json(payload): Json<BroadcastMessage>) -> StatusCode {
    if let Ok(handler) = manager.get_hotel_connection_handler(payload.hotel) {
        let session_service = handler.get_session_service();
        tokio::spawn(async move {
            session_service.broadcast(composer::RoomUserTalk { msg: payload.message }.compose()).await;
        });

        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}