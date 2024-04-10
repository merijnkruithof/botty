use std::sync::Arc;

use axum::{Extension, Json};
use http::StatusCode;
use serde::Serialize;

use crate::retro;

#[derive(Serialize)]
pub struct AvailableHotelItem {
    pub name: String,
}

#[derive(Serialize)]
pub struct AvailableHotel {
    items: Vec<AvailableHotelItem>
}

pub async fn list(connection_service: Extension<Arc<retro::Manager>>) -> Result<Json<AvailableHotel>, StatusCode> {
    if let Ok(retros) = connection_service.list_retros() {
        Ok(Json(AvailableHotel{
            items: retros.iter().map(|entry| AvailableHotelItem{ name: entry.clone() }).collect()
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}