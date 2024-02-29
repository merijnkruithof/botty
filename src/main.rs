use composer::Composable;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
};
use tokio_tungstenite::tungstenite::Message;

mod app_config;
mod client;
mod composer;
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

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let mut sessions: Vec<session::Session> = Vec::new();

    start_client_connections(&mut handles, &mut sessions).await;

    // handle the command line here
    for session in &sessions {
        crate::composer::ClientHello {}.send(&session.tx).await;

        crate::composer::AuthTicket {
            sso_ticket: &session.ticket,
        }
        .send(&session.tx)
        .await;
    }

    for handle in handles {
        handle.await?;
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
    handles: &mut Vec<JoinHandle<()>>,
    sessions: &mut Vec<session::Session>,
) {
    match app_config::load() {
        Ok(config) => {
            // Read all auth tickets from the config file and spawn client processes.
            for ticket in config.tickets {
                let uri = config.uri.clone();
                let auth_ticket = ticket.clone();

                let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel(1);
                let session = session::Session { ticket, tx };

                sessions.push(session);

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

async fn create_client_connection_handler(uri: String, auth_ticket: String, rx: Receiver<Message>) {
    match client::connect(uri).await {
        Ok(client) => {
            client::handle(client, rx)
                .await
                .expect("unable to handle client connection");

            println!(
                "Connection with auth ticket '{}' closed. This ticket is not usable anymore.",
                auth_ticket
            );
        }

        Err(err) => eprintln!(
            "an error occurred while trying to connect to the server: {}",
            err
        ),
    }
}
