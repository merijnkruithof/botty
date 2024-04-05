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

pub async fn list(connection_service: Extension<Arc<retro::Manager>>) -> (StatusCode, Json<AvailableHotel>) {
    let handlers = connection_service.list_retros();
    if handlers.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(AvailableHotel{ items: Vec::new() }));
    }

    let mut view = AvailableHotel{
        items: Vec::new()
    };

    let handlers = handlers.unwrap();
    for name in handlers.iter() {
        view.items.push(AvailableHotelItem{
            name: name.clone(),
        });
    }

    (StatusCode::OK, Json(view))
}