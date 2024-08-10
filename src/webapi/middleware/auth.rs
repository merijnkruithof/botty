use anyhow::{anyhow, Result};
use axum::{extract::Request, http, middleware::Next, response::Response};
use axum::extract::State;
use http::{HeaderMap, StatusCode};


use tracing::error;

use crate::webapi::app_state::AppState;

pub async fn handle(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if request.method().as_str() == "OPTIONS" {
        let response = next.run(request).await;
        return Ok(response);
    }

    match get_token_from_headers(&headers) {
        Ok(token) => {
            if token == state.auth_token {
                let response = next.run(request).await;
                return Ok(response);
            }

            error!("Invalid auth token provided");

            Err(StatusCode::UNAUTHORIZED)
        },

        Err(err) => {
            error!("unable to process request: {:?}", err);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn get_token_from_headers(headers: &HeaderMap) -> Result<&str> {
    if let Some(header_value) = headers.get("x-auth-token") {
        if let Ok(token) = header_value.to_str() {
            return Ok(token);
        }
    }

    Err(anyhow!("Auth token not found in header"))
}