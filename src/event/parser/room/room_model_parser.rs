use crate::communication::packet::Reader;
use crate::event::controller::room::RoomModelEvent;

pub fn parse(mut reader: Reader) -> RoomModelEvent {
    RoomModelEvent{
        model: reader.read_string().unwrap(),
        room_id: reader.read_uint32().unwrap(),
    }
}