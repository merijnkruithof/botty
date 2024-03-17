use std::sync::{Arc};
use axum::{Extension, Json};
use http::StatusCode;
use serde::Deserialize;
use tracing::error;
use crate::session;

#[derive(Deserialize)]
pub struct AddSession {
    hotel: String,
    auth_ticket: String,
}

pub async fn add(connection_service: Extension<Arc<crate::connection::Service>>, Json(payload): Json<AddSession>) -> StatusCode {
    return match connection_service.get_handler(payload.hotel) {
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
    };
}

#[derive(Deserialize)]
pub struct AddSessionMany {
    hotel: String,
    tickets: Vec<String>
}

pub async fn add_many(connection_service: Extension<Arc<crate::connection::Service>>, Json(payload): Json<AddSessionMany>) -> StatusCode {
    return match connection_service.get_handler(payload.hotel) {
        Ok(handler) => {
            for ticket in payload.tickets {
                if let Err(err) = handler.new_client(ticket.clone()).await {
                    error!("unable to add session {}, reason: {:?}", ticket.clone(), err);
                }
            }

            StatusCode::OK
        },

        Err(err)  => {
            error!("Unable to add sessions: {:?}", err);

            StatusCode::INTERNAL_SERVER_ERROR
        }
    };
}

#[derive(Deserialize)]
pub struct KillSession {
    hotel: String,
    auth_ticket: String,
}

pub async fn kill(connection_service: Extension<Arc<crate::connection::Service>>, Json(payload): Json<KillSession>) -> StatusCode {
    return match connection_service.get_handler(payload.hotel) {
        Ok(handler) => {
            if let Err(err) = handler.get_session_service().kill(payload.auth_ticket) {
                error!("Unable to kill session: {:?}", err);

                return StatusCode::BAD_REQUEST;
            }

            StatusCode::OK
        },

        Err(err)  => {
            error!("Unable to kill session: {:?}", err);

            StatusCode::INTERNAL_SERVER_ERROR
        }
    };
}