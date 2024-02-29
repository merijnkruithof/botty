use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::borrow::Borrow;
use tokio::{
    net::TcpStream,
    sync::{mpsc::Receiver, watch},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::Request, http::Uri, protocol::Message, Error},
    MaybeTlsStream, WebSocketStream,
};

pub async fn connect(ws_link: String) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Error> {
    let uri = Uri::try_from(ws_link).expect("invalid uri");

    let request = Request::builder()
        .uri(uri)
        .header("Origin", "http://localhost")
        .body(())
        .expect("Failed to build request");

    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");

    println!("WebSocket handshake has been successfully completed");

    Ok(ws_stream)
}

async fn handle_read(
    mut reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    shutdown_tx: tokio::sync::watch::Sender<bool>,
) {
    loop {
        let data = reader.next().await;
        if data.is_none() {
            shutdown_tx
                .send(true)
                .expect("unable to send data to the shutdown channel");

            break;
        }

        match data.unwrap() {
            Ok(msg) => {
                if msg.is_close() {
                    shutdown_tx
                        .send(true)
                        .expect("unable to send data to the shutdown channel");
                    break;
                }
            }

            Err(err) => {
                eprintln!("an error occurred while trying to read a packet: {:?}", err);

                shutdown_tx
                    .send(true)
                    .expect("unable to send data to the shutdown channel");

                break;
            }
        }
    }
}

async fn handle_write(
    mut writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut ch: tokio::sync::mpsc::Receiver<Message>,
    mut shutdown_rx: tokio::sync::watch::Receiver<bool>,
) {
    loop {
        tokio::select! {
            data = ch.recv() => {
                match data {
                    Some(msg) => {
                        if let Err(err) = writer.send(msg).await {
                            eprintln!("An error occurred while trying to send data to the websocket: {:?}", err);
                            continue;
                        }
                    },

                    None => break
                }
            },

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    println!("Writer closed, shutdown channel has been triggered");
                    break;
                }
            }
        }
    }
}

// Handle takes care of handling the client to server connection.
//
// It enables client -> server communication, as well as server -> client communication. It spawns
pub async fn handle(
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    ch: Receiver<Message>,
) -> Result<(), Error> {
    let (writer, reader) = stream.split();

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let read_handle = tokio::spawn(handle_read(reader, shutdown_tx));
    let write_handle = tokio::spawn(handle_write(writer, ch, shutdown_rx.clone()));

    let _ = tokio::try_join!(write_handle, read_handle);

    Ok(())
}
