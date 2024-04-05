use std::sync::Arc;

use anyhow::Result;

use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::connection::session::Service;

pub async fn broadcast_enter(room_id: u32, session_service: Arc<Service>) -> Result<()> {
    let _ = session_service.broadcast(composer::RequestRoomLoad{ room_id }.compose()).await;

    Ok(())
}