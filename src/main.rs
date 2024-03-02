use std::collections::HashMap;

use composer::Composable;
use futures_util::future::select_all;
use session::Session;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
};
use tokio_tungstenite::tungstenite::Message;

use crate::errors::ClientError;

mod app_config;
mod client;
mod composer;
mod errors;
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

    let mut session_service = session::Service::new(HashMap::new());

    start_client_connections(&mut handles, &mut session_service).await;

    for session in session_service.all() {
        // send client hello to the server
        session_service
            .send(session, &composer::ClientHello {}.compose())
            .await
            .expect("unable to send client hello packet to the server");

        session_service
            .send(
                session,
                &composer::AuthTicket {
                    sso_ticket: session.ticket.as_str(),
                }
                .compose(),
            )
            .await
            .expect("unable to send auth ticket packet to the server");
    }

    while !handles.is_empty() {
        let (result, _, remaining) = select_all(handles).await;

        match result {
            Ok(connection_finished) => match connection_finished {
                Ok(auth_ticket) => {
                    session_service.delete(&auth_ticket);

                    println!("Task with auth ticket '{}' has been stopped", auth_ticket)
                }

                Err(err) => {
                    session_service.delete(&err.auth_ticket);

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

    Ok(())
}

// Start client connections.
//
// This function reads through all auth tickets provided by the config and creates a separate
// thread for each of them.
//
// It will store all thread handles to &mut handles, and it will store all client sessions to &mut sessions.
async fn start_client_connections(
    handles: &mut Vec<JoinHandle<Result<String, ClientError>>>,
    session_service: &mut session::Service,
) {
    match app_config::load() {
        Ok(config) => {
            // Read all auth tickets from the config file and spawn client processes.
            for ticket in config.tickets {
                let uri = config.uri.clone();
                let auth_ticket = ticket.clone();

                let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel(1);
                let session = session::Session { ticket, tx };

                session_service.insert(session);

                handles.push(tokio::spawn(create_client_connection_handler(
                    uri,
                    auth_ticket,
                    rx,
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
) -> Result<String, ClientError> {
    match client::connect(uri).await {
        Ok(client) => {
            client::handle(client, rx)
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
