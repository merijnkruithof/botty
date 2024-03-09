use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use futures_util::future::select_all;
use tokio::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    task::JoinHandle,
};

use tokio_tungstenite::tungstenite::protocol::Message;

use crate::errors::ClientError;

mod api;
mod app_config;
mod client;
mod composer;
mod errors;
mod event;
mod packet;
mod session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ascii_art = r#" _  (`-') (`-')  _           (`-')  _  (`-').->            (`-').->
    \-.(OO ) ( OO).-/    .->    (OO ).-/  ( OO)_       .->    ( OO)_
    _.'    \(,------. ,---(`-') / ,---.  (_)--\_) ,--.(,--.  (_)--\_)
   (_...--'' |  .---''  .-(OO ) | \ /`.\ /    _ / |  | |(`-')/    _ /
   |  |_.' |(|  '--. |  | .-, \ '-'|_.' |\_..`--. |  | |(OO )\_..`--.
   |  .___.' |  .--' |  | '.(_/(|  .-.  |.-._)   \|  | | |  \.-._)   \
   |  |      |  `---.|  '-'  |  |  | |  |\       /\  '-'(_ .'\       /
   `--'      `------' `-----'   `--' `--' `-----'  `-----'    `-----'  "#;
    println!("{}", ascii_art);
    println!("Habbo Bot Commander - 2024 edition");
    println!("Developed by Merijn (Discord: merijnn)");
    println!("-------------------------------------------------------------------------------");

    let mut handles: Vec<JoinHandle<Result<String, ClientError>>> = Vec::new();
    let session_service = Arc::new(Mutex::new(session::Service::new()));

    start_client_connections(&mut handles, session_service.clone()).await;

    let thread_handle = tokio::spawn(threads_handler(handles, session_service.clone()));
    let webserver_handle = tokio::spawn(webserver_handler(session_service.clone()));

    tokio::try_join!(thread_handle, webserver_handle).expect("unable to join threads");

    Ok(())
}

async fn webserver_handler(session_service: Arc<Mutex<session::Service>>) {
    let app = Router::new()
        .route("/api/health", get(api::health::index))
        .route("/api/bots/available", get(api::bot::available))
        .route(
            "/api/bots/broadcast_message",
            post(api::bot::broadcast_message),
        )
        .layer(Extension(session_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:666").await.unwrap();

    println!("Started webserver on localhost:666");

    axum::serve(listener, app).await.unwrap();
}

async fn threads_handler(
    mut handles: Vec<JoinHandle<Result<String, ClientError>>>,
    session_service: Arc<Mutex<session::Service>>,
) {
    while !handles.is_empty() {
        let (result, _, remaining) = select_all(handles).await;

        match result {
            Ok(connection_finished) => match connection_finished {
                Ok(auth_ticket) => {
                    session_service.lock().await.delete(&auth_ticket).await;

                    println!("Task with auth ticket '{}' has been stopped. Session has been removed successfully.", auth_ticket)
                }

                Err(err) => {
                    session_service.lock().await.delete(&err.auth_ticket).await;

                    println!(
                        "Task with auth ticket '{}' stopped due to an error: {}",
                        &err.auth_ticket, err
                    );
                }
            },

            Err(err) => {
                println!("Client wasn't able to connect with the server: {}", err)
            }
        }

        handles = remaining;
    }
}

async fn start_client_connections(
    handles: &mut Vec<JoinHandle<Result<String, ClientError>>>,
    session_service: Arc<Mutex<session::Service>>,
) {
    match app_config::load() {
        Ok(config) => {
            // Read all auth tickets from the config file and spawn client processes.
            for ticket in config.tickets {
                let uri = config.uri.clone();
                let auth_ticket = ticket.clone();

                let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel(1);

                // create a new session
                let session = Arc::new(session::Session {
                    ticket,
                    tx: tx.clone(),
                });

                // insert session
                let mut write_lock = session_service.lock().await;

                write_lock.insert(session).await;

                println!("Adding session for auth ticket {}", &auth_ticket);

                handles.push(tokio::spawn(create_client_connection_handler(
                    uri,
                    auth_ticket,
                    rx,
                    tx,
                )));
            }
        }

        Err(e) => eprintln!("Failed to load config {}", e),
    }
}

async fn create_client_connection_handler(
    uri: String,
    auth_ticket: String,
    rx: Receiver<Message>,
    tx: Sender<Message>,
) -> Result<String, ClientError> {
    match client::connect(uri).await {
        Ok(client) => {
            client::handle(client, rx, tx, &auth_ticket)
                .await
                .expect("unable to handle client connection");

            println!(
                "Connection with auth ticket '{}' closed. This ticket is not usable anymore.",
                auth_ticket
            );

            Ok(auth_ticket)
        }

        Err(err) => {
            eprintln!(
                "An error occurred while trying to connect to the websocket server: {:?}",
                err
            );

            return Err(ClientError::new(
                "Unable to connect to the websocket server",
                auth_ticket,
            ));
        }
    }
}
