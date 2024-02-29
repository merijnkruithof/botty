use bytes::BytesMut;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

use crate::packet::writer::Writer;

pub trait Composable {
    async fn send(&self, sender: &Sender<Message>);
}

pub struct ClientHello {}

impl Composable for ClientHello {
    async fn send(&self, sender: &Sender<Message>) {
        let mut buf = BytesMut::new();
        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(4000);
        packet_writer.write_string("NITRO-1-6-6-HTML5");

        sender
            .send(Message::binary(buf.to_vec()))
            .await
            .expect("Unable to send client hello packet to channel");

        println!("Sent client hello to channel");
    }
}

pub struct AuthTicket<'a> {
    pub sso_ticket: &'a str,
}

impl<'a> Composable for AuthTicket<'a> {
    async fn send(&self, writer: &Sender<Message>) {
        let mut buf = BytesMut::new();

        let mut packet_writer = Writer::new(&mut buf);
        packet_writer.write_uint16(2419);
        packet_writer.write_string(self.sso_ticket);
        packet_writer.write_uint32(0); // TODO: last ticker time

        writer
            .send(Message::Binary(buf.to_vec()))
            .await
            .expect("unable to send auth ticket packet to channel");

        println!("Sent auth ticket to server");
    }
}
