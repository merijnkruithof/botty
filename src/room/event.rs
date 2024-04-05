#[derive(Clone, Debug)]
pub enum RoomEvent {
    UserRemovedFromRoom { room_unit_id: u32 }
}
