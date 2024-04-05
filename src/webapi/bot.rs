use std::sync::Arc;


use anyhow::Result;
use axum::{Extension, Json};
use axum::response::IntoResponse;

use http::StatusCode;


use serde::{Deserialize, Serialize};


use tracing::{error};

use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::core::taskmgr::task;

use crate::retro;
use crate::user::User;
use crate::webapi::actions::room;

#[derive(Serialize)]
pub struct BotInfo {
    pub user_id: u32,
    pub username: String,
    pub motto: String,
    pub figure: String,
    pub gender: String,
    pub sso_ticket: String
}

#[derive(Serialize)]
pub struct AvailableBots {
    n: usize,
    bots: Option<Vec<BotInfo>>
}

#[derive(Deserialize)]
pub struct AvailableBotsRequest {
    hotel: String,
}

pub async fn available(
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<AvailableBotsRequest>
) -> Result<impl IntoResponse, impl IntoResponse> {
    match connection_service.get_hotel_connection_handler(payload.hotel) {
        Ok(handler) => {
            let mut response = AvailableBots{
                n: handler.get_session_service().online_bots(),
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

            Ok((StatusCode::OK, Json(response)))
        },

        Err(_err) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

#[derive(Deserialize)]
pub struct BroadcastMessage {
    hotel: String,
    message: String,
}

pub async fn broadcast_message(
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<BroadcastMessage>,
) -> StatusCode {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let handler = handler.unwrap();

    let session_service = handler.get_session_service();
    let message_clone = payload.message.clone();

    tokio::spawn(async move {
        session_service.broadcast(composer::RoomUserTalk { msg: message_clone }.compose()).await;
    });

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct BroadcastEnterRoom {
    hotel: String,
    room_id: u32,
}

pub async fn broadcast_enter_room(
    connection_service: Extension<Arc<retro::Manager>>,
    Json(payload): Json<BroadcastEnterRoom>,
) -> StatusCode {
    let handler = connection_service.get_hotel_connection_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let handler = handler.unwrap();

    let session_service = handler.get_session_service();
    let room_id = payload.room_id.clone();

    tokio::spawn(async move {
        if let Err(err) = room::broadcast_enter(room_id, session_service).await {
            error!("Unable to enter room: {:?}", err);
        }
    });

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct BroadcastWalk {
    hotel: String,
    following_user_id: u32,
}

struct Followed {
    target_x: u32,
    target_y: u32,
}

pub async fn broadcast_walk(_task_manager: Extension<Arc<task::Manager>>, _connection_service: Extension<Arc<retro::Manager>>, Json(_payload): Json<BroadcastWalk>) -> StatusCode {
    return StatusCode::OK;
    // if task_manager.has_task("broadcast_walk".to_string()) {
    //     return StatusCode::IM_USED;
    // }
    //
    // let handler = connection_service.get_hotel_connection_handler(payload.hotel);
    // if handler.is_err() {
    //     return StatusCode::NOT_FOUND;
    // }
    //
    // let handler = handler.unwrap();
    // let session_service = handler.get_session_service();
    //
    // let follow_user_id = payload.following_user_id.clone();
    //
    // // We're about to follow a specific user id. There are two conditions:
    // // 1. If the user is moving for >= 5 tiles, we'll follow the user.
    // // 2. If the user has stopped moving at n where n < 5 tiles, we'll follow the user. We'll keep
    // //    track of the time, because the user shouldn't spam the "walking" packet.
    // let task_added = task_manager.add_task(KillableTask::new("broadcast_walk".to_string(), async move {
    //     let (tx, mut rx) = tokio::sync::mpsc::channel(1); // TODO: buffer size
    //
    //     for session in session_service.all() {
    //         let read_lock = session.room.read().await;
    //         let tx_clone = tx.clone();
    //
    //         match &*read_lock {
    //             Some(room) => {
    //                 if let Some(room_user) = room.get_user_by_user_id(&follow_user_id) {
    //                     let binding = room_user.clone();
    //                     let room_user = binding.read().await;
    //                     let mut subscriber = room_user.subscribe_to_events();
    //                     let session_clone = session.clone();
    //
    //                     tokio::spawn(async move {
    //                         loop {
    //                             tokio::select! {
    //                                 _ = tx_clone.closed() => {
    //                                     break;
    //                                 },
    //
    //                                 data = subscriber.recv() => {
    //                                     let event = data.unwrap();
    //                                     tx_clone.send((session_clone.clone(), event)).await.unwrap();
    //                                 }
    //                             }
    //                         }
    //                     });
    //                 }
    //             },
    //             None => {
    //                 continue;
    //             }
    //         };
    //
    //     }
    //
    //     loop {
    //         tokio::select! {
    //             data = rx.recv() => {
    //                 if let Some((session, UserEvent::UserMoved {x, y })) = data {
    //                     let msg = composer::WalkInRoom{ x, y }.compose();
    //
    //                     session_service.send(&session, msg).await.unwrap();
    //                 }
    //             }
    //         }
    //     }
    // }));
    //
    // return if task_added { StatusCode::OK } else { StatusCode::INTERNAL_SERVER_ERROR }
}
#[derive(Deserialize)]
pub struct BroadcastAbuseCfh {
    hotel: String,
    message: String,
    user_id: Option<i32>,
    room_id: u32,
}

pub async fn broadcast_cfh_abuse(retro_manager: Extension<Arc<retro::Manager>>, Json(payload): Json<BroadcastAbuseCfh>) -> StatusCode {
    let handler = retro_manager.get_hotel_connection_handler(payload.hotel);
    if handler.is_err() {
        return StatusCode::NOT_FOUND;
    }

    let handler = handler.unwrap();

    handler.get_session_service().broadcast(composer::ReportComposer{
        message: payload.message,
        topic: 35,
        room_id: payload.room_id,
        user_id: if payload.user_id.is_none() { -1 } else { payload.user_id.unwrap() },
    }.compose()).await;

    StatusCode::OK
}
