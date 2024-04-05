use crate::communication::packet::Reader;
use crate::event::controller::room::RoomLoadedEvent;

pub fn parse(mut reader: Reader) -> RoomLoadedEvent {
    let _ = reader.read_bool();

    RoomLoadedEvent{
        room_id: reader.read_uint32().unwrap()
    }
}