use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use http::StatusCode;
use serde::Deserialize;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::{Composable, Dance};
use crate::retro;
use crate::webapi::error_handling::ErrorResponse;

#[derive(Deserialize)]
pub struct RoomDanceRequest {
    hotel: String,
    all_bots_must_dance: Option<bool>,
    bots: Option<Vec<String>>,
    dance_id: u32,
}

pub async fn dance(
    Path(room_id): Path<u32>,
    manager: Extension<Arc<retro::Manager>>,
    Json(payload): Json<RoomDanceRequest>
) -> Result<(), ErrorResponse> {
    if let Ok(handler) = manager.get_hotel_connection_handler(payload.hotel) {
        let global_state = handler.global_state();
        let room_manager = global_state.room_manager.clone();

        if let Some(mut room_players) = room_manager.get_bots_in_room(room_id) {
            let bots_that_must_dance = payload.bots.unwrap_or_else(|| vec![]);
            let all_bots_must_dance = payload.all_bots_must_dance.unwrap_or_default();

            if !all_bots_must_dance {
                room_players = room_players
                    .iter()
                    .filter(|entry| bots_that_must_dance.contains(&entry.clone()))
                    .cloned()
                    .collect::<Vec<String>>();
            }

            let session_service = handler.get_session_service();
            tokio::spawn(async move {
                for room_user in room_players {
                    if let Some(session) = session_service.get(&room_user) {
                        let msg = composer::Dance{
                            dance_id: payload.dance_id.clone(),
                        }.compose();

                        let _ = session_service.send(&session, msg).await;
                    }
                }
            });

            Ok(())
        } else {
            Err(ErrorResponse::new("No bots in the room", StatusCode::CONFLICT))
        }
    } else {
        Err(ErrorResponse::new("Hotel not found", StatusCode::NOT_FOUND))
    }
}