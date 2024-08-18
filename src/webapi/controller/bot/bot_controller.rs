use std::collections::HashMap;
use std::sync::Arc;
use axum::{debug_handler, Extension, Json};
use axum::extract::Path;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::retro;
use axum;
use axum::response::IntoResponse;
use defer::defer;
use log::warn;
use tokio_tungstenite::tungstenite::connect;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Status;
use tracing::error;
use utoipa::{OpenApi, ToSchema};
use uuid::uuid;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::connection::session::Session;
use crate::core::taskmgr::task;
use crate::core::taskmgr::task::KillableTask;

#[derive(OpenApi)]
#[openapi(
    paths(index,show),
    components(schemas(AvailableBots,BotInfo,BotsRequest, ShowBotRequest,BulkUpdateResponse))
)]
pub struct BotApi;

#[derive(Serialize, ToSchema)]
pub struct AvailableBots {
    bots: Option<Vec<BotInfo>>
}

#[derive(Serialize, ToSchema)]
pub struct BotInfo {
    pub user_id: u32,
    pub username: String,
    pub motto: String,
    pub figure: String,
    pub gender: String,
    pub sso_ticket: String
}

#[derive(Deserialize, ToSchema)]
pub struct BotsRequest {
    hotel: String,
}

#[utoipa::path(
    get,
    path = "",
    description = "Get all online bots including all user info.",
    request_body(
        content = BotsRequest,
        description = "Payload to request bots based on the hotel",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "List of available bots", body = AvailableBots),
        (status = 400, description = "Bad request"),
    ),
    security(
        ("api_key" = [])
    ),

    tag = "Bot"
)]
pub async fn index(connection_service: Extension<Arc<retro::Manager>>, Json(payload): Json<BotsRequest>) -> Result<Json<AvailableBots>, StatusCode> {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel).map_err(|_| StatusCode::NOT_FOUND)?;

    let mut response = AvailableBots{
        bots: None,
    };

    let user_manager = handler.global_state().user_manager.clone();
    let bots: Vec<BotInfo> = user_manager.users().iter().map(|entry| {
        BotInfo{
            user_id: entry.user_id.clone(),
            username: entry.username.clone(),
            motto: entry.motto.clone(),
            figure: entry.figure.clone(),
            gender: entry.gender.clone(),
            sso_ticket: entry.sso_ticket.clone()
        }
    }).collect();

    if bots.len() > 0 {
        response.bots = Some(bots);
    }

    Ok(Json(response))
}

#[derive(Deserialize, ToSchema)]
pub struct ShowBotRequest {
    hotel: String,
}


#[utoipa::path(
    get,
    path = "/{ticket}",
    description = "Get a single bot's information.",
    request_body(
        content = ShowBotRequest,
        description = "Payload to request a single based on the hotel",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The requested bot", body = AvailableBots),
        (status = 400, description = "Bad request"),
    ),
    security(
        ("api_key" = [])
    ),

    tag = "Bot"
)]
pub async fn show(
    ticket: Path<String>,
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<ShowBotRequest>
) -> Result<Json<BotInfo>, StatusCode> {
    match connection_service.get_hotel_connection_handler(payload.hotel) {
        Ok(handler) => {
            let user_manager = handler.global_state().user_manager.clone();

            if let Some(bot) = user_manager.get_user(ticket.clone()) {
                Ok(Json(BotInfo {
                    user_id: bot.user_id.clone(),
                    username: bot.username.clone(),
                    motto: bot.motto.clone(),
                    figure: bot.figure.clone(),
                    gender: bot.gender.clone(),
                    sso_ticket: bot.sso_ticket.clone()
                }))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        },

        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateBot {
    hotel: String,
    motto: Option<String>,
    appearance: Option<UpdateAppearance>
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateAppearance {
    gender: String,
    figure: String
}

#[utoipa::path(
    put,
    path = "/{ticket}",
    description = "Update a single bot.",
    request_body(
        content = UpdateBot,
        description = "Payload to update a single bot.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The requested bot", body = AvailableBots),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Hotel or bot not found"),
    ),
    security(
        ("api_key" = [])
    ),

    tag = "Bot"
)]
pub async fn update(
    ticket: Path<String>,
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<UpdateBot>
) -> Result<(), StatusCode> {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel).map_err(|_| StatusCode::NOT_FOUND)?;

    let session_service = handler.get_session_service();

    if let Some(session) = session_service.get(&ticket) {
        if let Some(motto) = payload.motto {
            let _ = session_service.send(&session, composer::UpdateMotto { motto }.compose()).await;
        }

        if let Some(appearance) = payload.appearance {
            let _ = session_service.send(&session, composer::UpdateLook { figure: appearance.figure, gender: appearance.gender }.compose()).await;
        }

        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize, ToSchema)]
pub struct BulkUpdateBot {
    hotel: String,
    bots: HashMap<String, UpdatableBulk>
}

#[derive(Deserialize, ToSchema)]
pub struct UpdatableBulk {
    motto: Option<String>,
    appearance: Option<UpdateAppearance>
}

#[derive(Serialize, ToSchema)]
pub struct BulkUpdateResponse {
    task_id: String,
}

#[utoipa::path(
    put,
    path = "/bulk_update",
    description = "Update a batch of bots.",
    request_body(
        content = BulkUpdateBot,
        description = "Payload to update a bunch of bots at the same time.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Task ID", body = BulkUpdateResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Hotel not found"),
    ),
    security(
        ("api_key" = [])
    ),

    tag = "Bot"
)]
pub async fn bulk_update(
    connection_service: Extension<Arc<retro::Manager>>,
    task_manager: Extension<Arc<task::Manager>>,
    Json(payload): Json<BulkUpdateBot>
) -> Result<Json<BulkUpdateResponse>, StatusCode> {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel).map_err(|_| StatusCode::NOT_FOUND)?;

    let task_id = uuid::Uuid::new_v4().to_string();

    // Create cloned values for the thread
    let cloned_task_id = task_id.clone();
    let cloned_task_manager = task_manager.clone();
    let session_service = handler.get_session_service();
    task_manager.add_task(KillableTask::new(task_id.clone(), async move {
        for (sso_ticket, updatable) in payload.bots {
            if let Some(session) = session_service.get(&sso_ticket) {
                if let Some(motto) = updatable.motto {
                    let _ = session_service.send(&session, composer::UpdateMotto { motto}.compose()).await;
                }

                if let Some(appearance) = updatable.appearance {
                    let _ = session_service.send(&session, composer::UpdateLook { figure: appearance.figure, gender: appearance.gender }.compose()).await;
                }
            } else {
                error!("Bulk update task {}: unable to get session for auth ticket {}", &cloned_task_id, &sso_ticket);
            }
        }

        let _ = cloned_task_manager.kill_task(cloned_task_id).await;
    }));

    Ok(Json(BulkUpdateResponse { task_id }))
}