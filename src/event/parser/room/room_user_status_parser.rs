use crate::communication::packet::Reader;
use crate::event::controller::room::{RoomUnit, RoomUserStatusEvent};

pub fn parse(mut reader: Reader) -> RoomUserStatusEvent {
    let total_units = reader.read_uint32().unwrap() as usize;

    let mut units = Vec::with_capacity(total_units);

    for _ in 0..total_units {
        units.push(RoomUnit{
            room_unit_id: reader.read_uint32().unwrap(),
            x: reader.read_uint32().unwrap(),
            y: reader.read_uint32().unwrap(),
            z: reader.read_string().unwrap(),
            head_direction: reader.read_uint32().unwrap(),
            direction: reader.read_uint32().unwrap(),
            actions: reader.read_string().unwrap()
        });
    }

    RoomUserStatusEvent{
        total_units,
        room_units: units,
    }
}