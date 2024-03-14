use std::sync::{Arc};
use axum::{Extension, Json};
use http::StatusCode;
use serde::Serialize;
use tokio::sync::Mutex;
use crate::session;

#[derive(Serialize)]
pub struct Home {
    condition: &'static str,
    online_bots: usize,
}

pub async fn index(session_service: Extension<Arc<Mutex<session::Service>>>) -> (StatusCode, Json<Home>) {
    let read_lock = session_service.lock().await;
    let online_bots = read_lock.online_bots().await;

    let condition = if online_bots > 0 { "Healthy" } else { "Unhealthy" };
    (StatusCode::OK, Json(Home { condition, online_bots }))
}
