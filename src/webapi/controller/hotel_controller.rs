use std::sync::Arc;

use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tracing::error;
use crate::client::hotel;
use crate::{connection, retro};

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

#[derive(Deserialize)]
pub struct AddHotel {
    name: String,
    ws_link: String,
}

pub async fn add_hotel(connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<AddHotel>) -> Result<(), StatusCode> {
    let connector = connection::Connector::new(payload.name.clone(), payload.ws_link.clone());
    let hotel_manager = hotel::Manager::new(Arc::new(connector));

    let _ = connection_service.add_hotel(payload.name.clone(), Arc::new(hotel_manager)).await.map_err(|previous_err| {
        error!("Pegasus can't add hotel with name {} due to an error: {:?}", payload.name.clone(), previous_err);

        StatusCode::CONFLICT
    })?;

    Ok(())
}

#[derive(Deserialize)]
pub struct DeleteHotel {
    name: String,
}

pub async fn delete_hotel(connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<DeleteHotel>) -> Result<(), StatusCode> {
    let hotel_manager = connection_service.get_hotel_connection_handler(payload.name.clone()).map_err(|_| StatusCode::NOT_FOUND)?;

    // Disconnect each session
    for state in hotel_manager.bot_states() {
        let _ = state.packet_tx.send(Message::Close(None)).await.unwrap();
    }

    let _ = connection_service.delete_hotel_connection_handler(payload.name);

    Ok(())
}