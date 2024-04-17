use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Deserialize;
use tracing::error;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::retro;
use crate::webapi::error_handling::ErrorResponse;

#[derive(Deserialize)]
pub struct EnterRoomRequest {
    hotel: String,
    room_id: u32,
    all_bots_must_enter: Option<bool>,
    bots: Option<Vec<String>>
}

pub async fn enter_room(
    manager: Extension<Arc<retro::Manager>>,
    Json(payload): Json<EnterRoomRequest>
) -> Result<(), ErrorResponse> {
    match manager.get_hotel_connection_handler(payload.hotel) {
        Ok(handler) => {
            let msg = composer::RequestRoomLoad{ room_id: payload.room_id }.compose();
            let session_service = handler.get_session_service();

            let all_bots_must_enter = payload.all_bots_must_enter.unwrap_or_default();
            let bots = payload.bots.unwrap_or_else(|| vec![]);

            if bots.len() == 0 && !all_bots_must_enter {
                return Err(ErrorResponse::new("Specify a list with bots or set all_bots_must_enter", StatusCode::BAD_REQUEST));
            }

            if all_bots_must_enter {
                tokio::spawn(async move {
                    let _ = session_service.broadcast(msg).await;
                });

                return Ok(());
            }

            tokio::spawn(async move {
                for bot in bots {
                    if let Some(session) = session_service.get(&bot) {
                        let _ = session_service.send(&session, msg.clone()).await;
                    }
                }
            });

            Ok(())
        },

        Err(_) => {
            Err(ErrorResponse::new("Hotel not found", StatusCode::NOT_FOUND))
        }
    }
}