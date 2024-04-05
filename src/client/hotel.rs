use std::sync::Arc;

use anyhow::{anyhow, Result};
use dashmap::DashMap;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, watch};
use tracing::info;

use crate::{communication, connection};
use crate::client::state;
use crate::client::state::BotState;
use crate::connection::{session, Subscriber};

pub struct Manager {
    connector: Arc<connection::Connector>,
    global_state: Arc<state::GlobalState>,
    client_states: DashMap<String, Arc<BotState>>
}

impl Manager {
    pub fn new(connector: Arc<connection::Connector>) -> Self {
        Manager {
            connector,
            global_state: Arc::new(state::GlobalState::new()),
            client_states: DashMap::new(),
        }
    }

    pub fn get_session_service(&self) -> Arc<session::Service> {
        self.global_state.session_service.clone()
    }

    pub fn global_state(&self) -> Arc<state::GlobalState> {
        self.global_state.clone()
    }

    pub fn bot_states(&self) -> Vec<Arc<BotState>> {
        return self.client_states
            .iter()
            .map(|entry| entry.value().clone())
            .collect();
    }

    pub async fn new_client(&self, auth_ticket: String) -> Result<()>  {
        let session_service = self.global_state.session_service.clone();
        if session_service.has(&auth_ticket) {
            return Err(anyhow!("Session with auth ticket {} already exists", &auth_ticket));
        }

        // Establish a new websocket connection
        let connection = self.connector.connect().await?;

        let (writer, reader) = connection.split();
        let (cancellation_tx, cancellation_rx) = watch::channel(false);

        // Connection is established! We can now listen for incoming packets.
        // Set a rate limit of 100 packets
        let rate_limit = 100;
        let receiver = connection::Receiver::new(rate_limit);
        let recv_subscriber = receiver.subscribe();
        let receiver_handle = tokio::spawn(async move {
            let _ = receiver.receive(reader, cancellation_tx).await;
        });

        // Allow Pegasus to send packets to the server.
        let (packet_tx, packet_rx) = mpsc::channel(1);
        let mut sender = connection::Sender{};
        let sender_handle = tokio::spawn(async move {
            let _ = sender.send(packet_rx, writer, cancellation_rx.clone()).await;
        });

        // Create new bot state
        let bot_state = Arc::new(BotState::new(auth_ticket.clone(), packet_tx.clone(), recv_subscriber));
        self.client_states.insert(auth_ticket.clone(), bot_state.clone());

        session_service.add_session(bot_state.session.clone());

        // Start communication flow
        let communication = communication::Handler::new(self.global_state.clone(), bot_state.clone());
        let communication_handle = tokio::spawn(async move {
            let _ = communication.start().await;
        });

        tokio::try_join!(receiver_handle, sender_handle, communication_handle);

        self.client_states.remove(&auth_ticket);
        session_service.delete(&auth_ticket);

        info!("Removed client state and session state for {}", &auth_ticket);

        Ok(())
    }
}