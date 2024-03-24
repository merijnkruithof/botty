use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::{mpsc, RwLock, watch};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
use crate::{client, communication};
use anyhow::{anyhow, Result};
use dashmap::DashMap;
use defer::defer;
use tokio::runtime::Handle;
use tracing::{error, info};
use crate::client::session;
use crate::communication::incoming;

pub struct Builder {
    ws_link: Option<String>,
    origin: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Builder{ ws_link: None, origin: None }
    }

    pub fn with_ws_config(&mut self, ws_link: String, origin: String) -> &mut Builder {
        self.ws_link = Some(ws_link);
        self.origin = Some(origin);

        self
    }

    pub fn build(&self) -> Result<Manager> {
        let ws_link = self.ws_link.clone().ok_or_else(|| anyhow!("Provide a websocket link"))?;
        let origin = self.origin.clone().ok_or_else(|| anyhow!("Provide a websocket origin"))?;

        // Create a new connector instance based on the above configuration.
        let connector = Arc::new(communication::connection::Connector::new(ws_link, origin));

        // Create a new session service for this site.
        let session_factory = Arc::new(session::Factory::new());
        let session_service = Arc::new(session::Service::new());

        // Create a new event handler for this site.
        let event_handler = Arc::new(incoming::message::Handler::new(
            Arc::new(incoming::controller::handler::Factory::new()),
            session_service.clone())
        );

        // Create a new connection handler for this site.
        let connection_handler = Arc::new(communication::connection::Handler::new(
            Arc::new(communication::connection::Receiver::new(event_handler)),
            Arc::new(communication::connection::Sender::new())
        ));

        Ok(Manager::new(session_service, session_factory, connector, connection_handler))
    }
}

// Manager takes full responsibility of adding new bots to the hotel, removing bots, accessing
// session data, and much more. It's the entrypoint for a specific hotel, e.g. "Localhost Hotel".
pub struct Manager {
    session_service: Arc<session::Service>,
    session_factory: Arc<session::Factory>,
    connector: Arc<communication::connection::Connector>,
    connection_handler: Arc<communication::connection::Handler>
}

impl Manager {
    pub fn new(
        session_service: Arc<session::Service>,
        session_factory: Arc<session::Factory>,
        connector: Arc<communication::connection::Connector>,
        connection_handler: Arc<communication::connection::Handler>
    ) -> Self {
        Manager {
            session_service,
            session_factory,
            connector,
            connection_handler
        }
    }

    pub fn get_session_service(&self) -> Arc<session::Service> {
        return self.session_service.clone();
    }

    pub async fn new_client(&self, auth_ticket: String) -> Result<()>  {
        if self.session_service.has(&auth_ticket) {
            return Err(anyhow!("Session already exists"))
        }

        // Each session needs a connection reader and writer for handling incoming packet data.
        // Create a new channel for packet communication.
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel(1);

        // Create a cancellation token for the threads that are related to this connection. Calling
        // this token will kill every thread that's related to this client.
        let (kill_sig_tx, kill_sig_rx) = watch::channel(false);

        // Create a new session object
        let session = self.session_factory.make_session(
            auth_ticket.clone(),
            tx.clone(),
            kill_sig_rx.clone()
        );

        // Add a new session.
        self.session_service.add_session(session.clone());

        info!("Created session {}", &session.ticket);

        return match self.connector.connect().await {
            Ok(client) => {
                info!("Created connection for auth ticket {}", &session.ticket);

                // Handle the connection. This is a long-blocking call, so waiting for this is more
                // than expected.
                let resp = self.connection_handler.handle(client, session.clone(), rx).await;

                if let Err(err) = resp {
                    error!("An error occurred while handling the connection: {:?}", err);
                }

                self.session_service.delete(&session.ticket);

                kill_sig_tx.send(true).unwrap();

                info!("Session {} removed", &session.ticket);

                Ok(())
            },

            Err(err) => {
                self.session_service.delete(&session.ticket);

                kill_sig_tx.send(true).unwrap();

                info!("Session {} removed", &session.ticket);

                Err(err)
            }
        };
    }
}