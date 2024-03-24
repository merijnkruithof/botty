use std::sync::Arc;
use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;
use crate::client::hotel::Builder;
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

#[derive(Deserialize)]
pub struct AddHotel {
    name: String,
    ws_link: String,
    origin: String,
}

pub async fn add_hotel(connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<AddHotel>) -> StatusCode {
    let handler = Builder::new()
        .with_ws_config(payload.ws_link, payload.origin)
        .build();

    if let Err(err) = handler {
        error!("Unable to build hotel manager: {:?}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let handler = Arc::new(handler.unwrap());

    if let Err(err) = connection_service.add_hotel(payload.name, handler).await {
        error!("Unable to add hotel: {:?}", err);

        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}