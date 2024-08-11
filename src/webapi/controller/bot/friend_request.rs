use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use http::StatusCode;
use serde::Deserialize;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::retro;
use crate::webapi::error_handling::ErrorResponse;

#[derive(Deserialize)]
pub struct SendFriendRequest {
    hotel: String,
    username: String,
}

pub async fn broadcast_send_friend_request(manager: Extension<Arc<retro::Manager>>, Json(payload): Json<SendFriendRequest>) -> Result<(), ErrorResponse> {
    let handler = manager.get_hotel_connection_handler(payload.hotel)
        .map_err(|_| ErrorResponse::new("Hotel not found", StatusCode::NOT_FOUND))?;

    let session_service = handler.get_session_service();

    tokio::spawn(async move {
        let msg = composer::FriendRequest{ username: payload.username }.compose();
        let _ = session_service.broadcast(msg).await;
    });

    Ok(())
}

pub async fn send_friend_request(Path(bot_id): Path<String>, manager: Extension<Arc<retro::Manager>>, Json(payload): Json<SendFriendRequest>) -> Result<(), ErrorResponse> {
    let handler = manager.get_hotel_connection_handler(payload.hotel)
        .map_err(|_| ErrorResponse::new("Hotel not found", StatusCode::NOT_FOUND))?;

    let session_service = handler.get_session_service();

    if let Some(bot) = session_service.get(&bot_id) {
        let msg = composer::FriendRequest{ username: payload.username }.compose();

        let _ = session_service.send(&bot, msg).await;

        Ok(())
    } else {
        Err(ErrorResponse::new("Bot not found", StatusCode::CONFLICT))
    }
}