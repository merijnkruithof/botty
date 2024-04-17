use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorContent {
    error: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: ErrorContent,
    status_code: u16
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>, status_code: StatusCode) -> Self {
        let error = ErrorContent { error: error.into() } ;
        let status_code = status_code.as_u16();

        ErrorResponse { error, status_code }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(self.error);
        let status_code = StatusCode::from_u16(self.status_code).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        (status_code, body).into_response()
    }
}