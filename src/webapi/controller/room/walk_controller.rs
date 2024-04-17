use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use http::StatusCode;
use serde::{Deserialize};
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::retro;
use crate::webapi::error_handling::ErrorResponse;

#[derive(Deserialize)]
pub struct WalkToPositionRequest {
    hotel: String,
    position: Position,
    all_bots_must_walk: Option<bool>,
    bots: Option<Vec<String>>
}

#[derive(Deserialize)]
pub struct Position {
    x: u32,
    y: u32
}

pub async fn walk_to_position(
    Path(room_id): Path<u32>,
    manager: Extension<Arc<retro::Manager>>,
    Json(payload): Json<WalkToPositionRequest>
) -> Result<(), ErrorResponse> {
    match manager.get_hotel_connection_handler(payload.hotel) {
        Ok(handler) => {
            // Get all bots that must walk in the room
            let bots_that_must_enter_room = payload.bots.unwrap_or_else(|| vec![]);

            // Get the room manager
            let room_manager = handler.global_state().room_manager.clone();

            // Get all room users
            if let Some(mut room_users) = room_manager.get_bots_in_room(room_id) {
                let all_bots_must_walk = payload.all_bots_must_walk.unwrap_or_default();
                if !all_bots_must_walk {
                    room_users = room_users
                        .iter()
                        .filter(|entry| bots_that_must_enter_room.contains(&entry.clone()))
                        .cloned()
                        .collect::<Vec<String>>();
                }

                // Get the session service
                let session_service = handler.get_session_service();

                tokio::spawn(async move {
                    for room_user in room_users {
                        if let Some(session) = session_service.get(&room_user) {
                            let msg = composer::WalkInRoom{
                                x: payload.position.x,
                                y: payload.position.y
                            };

                            let _  = session_service.send(&session, msg.compose()).await;
                        }
                    }
                });

                Ok(())
            } else {
                Err(ErrorResponse::new("No bots in the room", StatusCode::CONFLICT))
            }
        },

        Err(_) => {
            Err(ErrorResponse::new("Hotel not found", StatusCode::NOT_FOUND))
        }
    }
}