use std::sync::{Arc};
use axum::{Extension, Json};
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct Home {
    condition: &'static str,
    online_bots: usize,
}

pub async fn index() -> (StatusCode, Json<Home>) {
    let online_bots = 1;

    let condition = if online_bots > 0 { "Healthy" } else { "Unhealthy" };
    (StatusCode::OK, Json(Home { condition, online_bots }))
}
