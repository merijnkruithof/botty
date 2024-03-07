use std::sync::{Arc, Mutex};

use axum::{Extension, Json};
use http::StatusCode;
use serde::Serialize;

use crate::session;

#[derive(Serialize)]
pub struct Home {
    message: &'static str,
}

pub async fn home() -> (StatusCode, Json<Home>) {
    (StatusCode::OK, Json(Home { message: "OK" }))
}

#[derive(Serialize)]
pub struct AvailableBots {
    n: usize,
}

pub async fn available_bots(
    session_service: Extension<Arc<Mutex<session::Service>>>,
) -> (StatusCode, Json<AvailableBots>) {
    let read_lock = session_service.lock().unwrap();

    (
        StatusCode::OK,
        Json(AvailableBots {
            n: read_lock.online_bots(),
        }),
    )
}
