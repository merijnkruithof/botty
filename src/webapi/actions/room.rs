use std::sync::Arc;

use anyhow::Result;

use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::connection::session::Service;

pub async fn broadcast_enter(room_id: u32, session_service: Arc<Service>) -> Result<()> {
    let sessions = session_service.all();

    for session in sessions {
        let room_id_clone = room_id.clone();
        let session_clone = session.clone();
        let session_service_clone = session_service.clone();

        tokio::spawn(async move{
            session_service_clone.send(&session_clone, composer::RequestRoomLoad { room_id: room_id_clone }.compose()).await.unwrap();
            session_service_clone.send(&session_clone, composer::RequestRoomData { room_id: room_id_clone }.compose()).await.unwrap();
        });
    }

    Ok(())
}