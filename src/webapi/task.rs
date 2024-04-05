use std::sync::Arc;

use axum::{Extension, Json};
use http::StatusCode;
use serde::Deserialize;

use crate::core::taskmgr::task;

#[derive(Deserialize)]
pub struct KillTask {
    hotel: String,
    task_id: String,
}

pub async fn kill_task(task_manager: Extension<Arc<task::Manager>>, Json(payload): Json<KillTask>) -> StatusCode {
    if !task_manager.has_task(payload.task_id.clone()) {
        return StatusCode::NOT_FOUND;
    }

    task_manager.kill_task(payload.task_id).await;

    StatusCode::OK
}
