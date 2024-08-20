use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use http::StatusCode;
use serde::Deserialize;
use tracing::{error, info};

use crate::retro;

#[derive(Deserialize)]
pub struct AddSession {
    auth_ticket: String,
}

pub async fn add(hotel: Path<String>, connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<AddSession>) -> StatusCode {
    match connection_service.get_hotel_connection_handler(hotel.clone()) {
        Ok(handler) => {
            if let Err(err) = handler.new_client(payload.auth_ticket.clone()).await {
                error!("unable to add session {}, reason: {:?}", payload.auth_ticket, err);

                return StatusCode::INTERNAL_SERVER_ERROR;
            }

            StatusCode::OK
        },

        Err(err)  => {
            error!("Unable to add auth ticket: {:?}", err);

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Deserialize)]
pub struct AddSessionMany {
    tickets: Vec<String>
}

pub async fn add_many(Path(hotel): Path<String>, connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<AddSessionMany>) -> StatusCode {
    match connection_service.get_hotel_connection_handler(hotel.clone()) {
        Ok(handler) => {
            for ticket in payload.tickets {
                let handler_clone = handler.clone();
                let ticket_clone = ticket.clone();

                tokio::spawn(async move {
                    if let Err(err) = handler_clone.new_client(ticket_clone).await {
                        error!("unable to add session, reason: {:?}", err);
                    }
                });
            }

            StatusCode::OK
        },

        Err(err)  => {
            error!("Unable to add sessions: {:?}", err);

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
