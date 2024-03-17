use std::sync::Arc;
use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;
use crate::connection;
use crate::connection::Config;

#[derive(Serialize)]
pub struct AvailableHotelItem {
    pub name: String,
    pub ws_link: String,
    pub origin: String,
}
#[derive(Serialize)]
pub struct AvailableHotel {
    items: Vec<AvailableHotelItem>
}

pub async fn list(connection_service: Extension<Arc<connection::Service>>) -> (StatusCode, Json<AvailableHotel>) {
    let handlers = connection_service.list_handlers();
    if handlers.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(AvailableHotel{ items: Vec::new() }));
    }

    let mut view = AvailableHotel{
        items: Vec::new()
    };

    let handlers = handlers.unwrap();
    for entry in handlers.iter() {
        view.items.push(AvailableHotelItem{
            name: entry.name.clone(),
            ws_link: entry.config.ws_link.clone(),
            origin: entry.config.origin.clone()
        });
    }

    (StatusCode::OK, Json(view))
}

#[derive(Deserialize)]
pub struct AddHotel {
    name: String,
    ws_link: String,
    origin: String,
}

pub async fn add_hotel(connection_service: Extension<Arc<connection::Service>>, Json(payload): Json<AddHotel>) -> StatusCode {
    let config = Config{
        ws_link: payload.ws_link,
        origin: payload.origin
    };

    if let Err(err) = connection_service.make_handler(payload.name, config).await {
        error!("Unable to add hotel: {:?}", err);

        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}