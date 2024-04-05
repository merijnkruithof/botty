// use crate::communication::packet::Reader;
// use crate::event::controller::room_load::RoomLoadedEvent;
// use anyhow::{anyhow, Result};
//
// pub fn parse_room_load(reader: &mut Reader, event: &mut RoomLoadedEvent) -> Result<()> {
//     let _ = reader.read_bool();
//
//     event.room_id = reader.read_uint32().unwrap();
//
//     Ok(())
// }