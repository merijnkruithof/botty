use std::sync::{Arc};
use axum::{Extension, Json};
use http::StatusCode;
use serde::Deserialize;
use tokio::sync::Mutex;
use crate::session;

#[derive(Deserialize)]
pub struct AddSession {
    auth_ticket: String,
}

pub async fn add(connection_service: Extension<Arc<crate::connection::Service>>, Json(payload): Json<AddSession>) -> StatusCode {
    if let Err(err) = connection_service.new_client(payload.auth_ticket).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct KillSession {
    auth_ticket: String,
}

pub async fn kill(session_service: Extension<Arc<Mutex<session::Service>>>, Json(payload): Json<KillSession>) -> StatusCode {
    let read_lock = session_service.lock().await;

    if let Err(err) = read_lock.kill(payload.auth_ticket).await {
        eprintln!("{:?}", err);

        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}