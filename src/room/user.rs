#[derive(Clone, Debug)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub figure: String,
    pub room_unit_id: u32,
    pub x: u32,
    pub y: u32,
    pub z: String,
    pub direction: u32
}