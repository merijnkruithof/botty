use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use serde::Deserialize;
use crate::retro;
use crate::webapi::error_handling::ErrorResponse;

#[derive(Deserialize)]
pub struct RoomActionRequest {
    hotel: String,
    all_bots_must_do_action: Option<bool>,
    bots: Option<Vec<String>>
}

pub async fn act(
    Path(room_id): Path<u32>,
    manager: Extension<Arc<retro::Manager>>,
    Json(payload): Json<RoomActionRequest>
) -> Result<(), ErrorResponse> {
    Ok(())
}