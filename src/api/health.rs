use std::sync::{Arc};
use axum::{Extension, Json};
use http::StatusCode;
use serde::Serialize;
use crate::session;

#[derive(Serialize)]
pub struct Home {
    condition: &'static str,
    online_bots: usize,
}

pub async fn index(session_service: Extension<Arc<session::Service>>) -> (StatusCode, Json<Home>) {
    let online_bots = session_service.online_bots();

    let condition = if online_bots > 0 { "Healthy" } else { "Unhealthy" };
    (StatusCode::OK, Json(Home { condition, online_bots }))
}
