use std::sync::{Arc};
use axum::{Extension, Json};
use http::StatusCode;
use serde::Deserialize;
use crate::session;

#[derive(Deserialize)]
pub struct AddSession {
    auth_ticket: String,
}

pub async fn add(connection_service: Extension<Arc<crate::connection::Service>>, Json(payload): Json<AddSession>) -> StatusCode {
    if let Err(err) = connection_service.new_client(payload.auth_ticket.clone()).await {
        tracing::error!("unable to add session {}, reason: {:?}", payload.auth_ticket, err);

        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct KillSession {
    auth_ticket: String,
}

pub async fn kill(session_service: Extension<Arc<session::Service>>, Json(payload): Json<KillSession>) -> StatusCode {
    if let Err(err) = session_service.kill(payload.auth_ticket.clone()) {
        tracing::error!("unable to kill session {}, reason: {:?}", payload.auth_ticket, err);

        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}