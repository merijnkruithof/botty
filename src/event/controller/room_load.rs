// use std::sync::Arc;
// use crate::communication::packet::Reader;
// use crate::connection::session::Session;
//
// pub struct RoomLoadedEvent {
//     pub room_id: u32,
// }
//
// pub enum RoomLoadedEvents {
//     RoomLoaded { event: RoomLoadedEvent },
// }
//
// pub struct RoomLoadedHandler { }
//
// impl RoomLoadedHandler {
//     fn handle(&self, session: Arc<Session>, mut reader: Reader) -> anyhow::Result<()> {
//         let mut room_loaded_event = RoomLoadedEvent{
//             room_id: 0,
//         };
//
//         let room_loaded_event = parse_room_load(&mut reader, &mut room_loaded_event)?;
//
//         // Dispatch RoomLoadedEvent
//
//         Ok(())
//     }
// }