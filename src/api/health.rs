use axum::Json;
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct Home {
    message: &'static str,
}

pub async fn index() -> (StatusCode, Json<Home>) {
    (StatusCode::OK, Json(Home { message: "OK" }))
}
