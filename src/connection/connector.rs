use std::time::Duration;

use anyhow::anyhow;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::tungstenite::http::Uri;

pub struct Connector {
    pub ws_link: String,
    pub origin: String,
}

impl Connector {
    pub fn new(ws_link: String, origin: String) -> Self {
        Connector{ws_link, origin }
    }

    pub async fn connect(&self) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let uri = Uri::try_from(self.ws_link.clone())?;

        let request = Request::builder()
            .uri(uri)
            .header("Origin", self.origin.clone())
            .body(())
            .expect("Failed to build request");

        let connect_future = connect_async(request);

        match timeout(Duration::from_secs(2), connect_future).await {
            Ok(Ok((ws_stream, _))) => Ok(ws_stream),
            Ok(Err(e)) => Err(anyhow!("WebSocket connection error: {}", e)),
            Err(_) => Err(anyhow!("WebSocket connection timeout")),
        }
    }
}