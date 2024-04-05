// use std::sync::Arc;
// use anyhow::anyhow;
// use crate::client::hotel;
// use crate::communication::outgoing::composer;
// use crate::communication::outgoing::composer::Composable;
// use crate::communication::packet::Reader;
// use crate::connection::session::Session;
// use crate::event::controller::handler::Handler;
//
// pub struct RoomModelHandler {
//     hotel_manager: Arc<hotel::Manager>
// }
//
//
// impl Handler for RoomModelHandler {
//     async fn handle(&self, session: Arc<Session>, reader: Reader) -> anyhow::Result<()> {
//         let session_service = self.hotel_manager.get_session_service();
//
//         session_service.send(&session, composer::RequestRoomHeightmap{}.compose())
//             .await
//             .map_err(|err| anyhow!("Unable to send room handler packet: {:?}", err))
//     }
// }