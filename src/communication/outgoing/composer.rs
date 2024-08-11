use bytes::BytesMut;
use tokio_tungstenite::tungstenite::Message;

use crate::communication::packet::Writer;

pub trait Composable {
    fn compose(&self) -> Message;
}

pub struct ClientHello {}

impl Composable for ClientHello {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();
        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(4000);
        packet_writer.write_string("NITRO-1-6-6-HTML5");

        Message::Binary(buf.to_vec())
    }
}

pub struct AuthTicket<'a> {
    pub sso_ticket: &'a str,
}

impl<'a> Composable for AuthTicket<'a> {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(2419);
        packet_writer.write_string(self.sso_ticket);
        packet_writer.write_uint32(0); // TODO: last ticker time

        Message::binary(buf.to_vec())
    }
}

pub struct RequestUserData { }

impl Composable for RequestUserData {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();
        let mut packet_writer = Writer::new(&mut buf);

        packet_writer.write_uint16(357);

        Message::binary(buf.to_vec())
    }
}

pub struct Pong {}

impl Composable for Pong {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(2596);

        Message::binary(buf.to_vec())
    }
}

pub struct RoomUserTalk {
    pub msg: String,
}

impl Composable for RoomUserTalk {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(1314);
        packet_writer.write_string(self.msg.as_str());

        Message::binary(buf.to_vec())
    }
}

pub struct RequestRoomLoad {
    pub room_id: u32,
}

impl Composable for RequestRoomLoad {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(2312);
        packet_writer.write_uint32(self.room_id);
        packet_writer.write_string("");

        Message::binary(buf.to_vec())
    }
}

pub struct RequestRoomHeightmap {}

impl Composable for RequestRoomHeightmap {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(2300);

        Message::binary(buf.to_vec())
    }
}

pub struct FloorPlanEditorRequestDoorSettings {}

impl Composable for FloorPlanEditorRequestDoorSettings {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(3559);

        Message::binary(buf.to_vec())
    }
}

pub struct FloorPlanEditorRequestBlockedTiles {}

impl Composable for FloorPlanEditorRequestBlockedTiles {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(1687);

        Message::binary(buf.to_vec())
    }
}

pub struct RequestRoomData {
    pub room_id: u32,
}

impl Composable for RequestRoomData {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(2230);
        packet_writer.write_uint32(self.room_id);
        packet_writer.write_uint32(0);

        Message::binary(buf.to_vec())
    }
}

pub struct WalkInRoom {
    pub x: u32,
    pub y: u32,
}

impl Composable for WalkInRoom {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(3320);
        packet_writer.write_uint32(self.x);
        packet_writer.write_uint32(self.y);

        Message::binary(buf.to_vec())
    }
}

pub struct ReportComposer {
    pub message: String,
    pub topic: u32,
    pub user_id: i32,
    pub room_id: u32
}

impl Composable for ReportComposer {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);

        packet_writer.write_uint16(1691);
        packet_writer.write_string(self.message.as_str());
        packet_writer.write_uint32(self.topic);
        packet_writer.write_int32(self.user_id);
        packet_writer.write_uint32(self.room_id);
        packet_writer.write_uint32(0); // message_count

        Message::binary(buf.to_vec())
    }
}

pub struct UpdateMotto {
    pub motto: String,
}

impl Composable for UpdateMotto {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();
        let mut writer = Writer::new(&mut buf);

        writer.write_uint16(2228);
        writer.write_string(self.motto.as_str());

        Message::binary(buf.to_vec())
    }
}

pub struct UpdateLook {
    pub figure: String,
    pub gender: String
}

impl Composable for UpdateLook {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();
        let mut writer = Writer::new(&mut buf);

        writer.write_uint16(2730);
        writer.write_string(self.gender.as_str());
        writer.write_string(self.figure.as_str());
        
        Message::binary(buf.to_vec())
    }
}

pub struct Dance {
    pub dance_id: u32
}

impl Composable for Dance {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();
        let mut writer = Writer::new(&mut buf);

        writer.write_uint16(2080);
        writer.write_uint32(self.dance_id);

        Message::binary(buf.to_vec())
    }
}

pub struct FriendRequest {
    pub username: String,
}

impl Composable for FriendRequest {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();
        let mut writer = Writer::new(&mut buf);

        writer.write_uint16(3157);
        writer.write_string(self.username.as_str());

        Message::binary(buf.to_vec())
    }
}