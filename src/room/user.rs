#[derive(Clone, Debug, Eq, Hash)]
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

impl PartialEq<Self> for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}
