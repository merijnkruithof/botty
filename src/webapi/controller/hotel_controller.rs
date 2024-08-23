use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tracing::error;
use utoipa::{OpenApi, ToSchema};
use crate::client::hotel;
use crate::{connection, retro};


#[derive(OpenApi, Debug)]
#[openapi(
    paths(list,add_hotel,delete_hotel),
    components(schemas(AvailableHotelItem,AvailableHotel,AddHotel))
)]
pub(crate) struct HotelApi;

#[derive(Serialize, ToSchema)]
pub struct AvailableHotelItem {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct AvailableHotel {
    items: Vec<AvailableHotelItem>
}

#[utoipa::path(
    get,
    path = "/",
    description = "Get all available hotels.",
    responses(
        (status = 200, description = "Available hotels", body = AvailableHotel),
        (status = 404, description = "Bad request"),
    ),
    security(
        ("api_key" = [])
    ),

    tag = "Hotel"
)]
pub async fn list(connection_service: Extension<Arc<retro::Manager>>) -> Result<Json<AvailableHotel>, StatusCode> {
    if let Ok(retros) = connection_service.list_retros() {
        Ok(Json(AvailableHotel{
            items: retros.iter().map(|entry| AvailableHotelItem{ name: entry.clone() }).collect()
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize, ToSchema)]
pub struct AddHotel {
    name: String,
    ws_link: String,
    origin: String,
    test_connection: bool,
}

#[utoipa::path(
    post,
    path = "/",
    description = "Add a new retro hotel to Pegasus.",
    request_body(
        content = AddHotel,
        description = "Add a new hotel to Pegasus",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Hotel added"),
        (status = 409, description = "Hotel already exists"),
        (status = 502, description = "Unable to connect to websocket server"),
    ),
    security(
        ("api_key" = [])
    ),

    tag = "Hotel"
)]
pub async fn add_hotel(connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<AddHotel>) -> Result<(), StatusCode> {
    let connector = connection::Connector::new(payload.ws_link.clone(), payload.origin.clone());
    if payload.test_connection {
        let _ =  connector.connect().await.map_err(|err| {
            error!("Unable to connect to the websocket server {} with origin {}, error: {:?}", payload.ws_link, payload.origin, err);

            StatusCode::BAD_GATEWAY
        })?;
    }

    let hotel_manager = hotel::Manager::new(Arc::new(connector));

    let _ = connection_service.add_hotel(payload.name.clone(), Arc::new(hotel_manager)).await.map_err(|previous_err| {
        error!("Pegasus can't add hotel with name {} due to an error: {:?}", payload.name.clone(), previous_err);

        StatusCode::CONFLICT
    })?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/{name}",
    description = "Delete an existing hotel.",
    responses(
        (status = 200, description = "Hotel deleted"),
        (status = 404, description = "Hotel not found"),
    ),
    security(
        ("api_key" = [])
    ),

    tag = "Hotel"
)]
pub async fn delete_hotel(Path(name): Path<String>, connection_service: Extension<Arc<retro::Manager>>) -> Result<(), StatusCode> {
    let hotel_manager = connection_service.get_hotel_connection_handler(name.clone()).map_err(|_| StatusCode::NOT_FOUND)?;

    // Disconnect each bot
    for state in hotel_manager.bot_states() {
        let _ = state.packet_tx.send(Message::Close(None)).await.unwrap();
    }

    let _ = connection_service.delete_hotel_connection_handler(name);

    Ok(())
}