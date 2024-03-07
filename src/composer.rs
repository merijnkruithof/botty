use bytes::BytesMut;
use tokio_tungstenite::tungstenite::Message;

use crate::packet::Writer;

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

pub struct Pong {}

impl Composable for Pong {
    fn compose(&self) -> Message {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(2596);

        Message::binary(buf.to_vec())
    }
}
