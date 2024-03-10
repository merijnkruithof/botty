use axum::Extension;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    composer::{self, Composable},
    session,
};

pub async fn broadcast_enter_room(
    room_id: u32,
    session_service: Extension<Arc<Mutex<session::Service>>>,
) {
    let read_lock = session_service.lock().await;

    read_lock
        .broadcast(composer::RequestRoomLoad { room_id }.compose())
        .await;

    read_lock
        .broadcast(composer::RequestRoomHeightmap {}.compose())
        .await;

    read_lock
        .broadcast(composer::FloorPlanEditorRequestDoorSettings {}.compose())
        .await;

    read_lock
        .broadcast(composer::FloorPlanEditorRequestBlockedTiles {}.compose())
        .await;

    read_lock
        .broadcast(composer::RequestRoomData { room_id }.compose())
        .await;
}
