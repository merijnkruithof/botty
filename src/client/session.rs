use std::sync::Arc;
use anyhow::anyhow;
use dashmap::DashMap;

use tokio::sync::{mpsc, mpsc::Receiver, mpsc::Sender, watch};
use tokio_tungstenite::tungstenite::Message;
use tracing::error;
use std::{fmt};
use std::sync::atomic::AtomicBool;

use tokio::sync::{RwLock};
use crate::client::session;

pub struct Factory {}

impl Factory {
    pub fn new() -> Self {
        Factory{}
    }

    pub fn make_session(&self, ticket: String, packet_tx: Sender<Message>, kill_sig_rx: watch::Receiver<bool>) -> Arc<Session> {
        Arc::new(Session {
            ticket,
            packet_tx,
            kill_sig_rx
        })
    }
}

pub struct Session {
    // ticket contains the authentication ticket of a session. This is currently its unique
    // identifier.
    pub ticket: String,

    // packet_tx is a channel for sending packets to the underlying network stream.
    pub packet_tx: Sender<Message>,

    // kill_sig_rx contains true if the connection has been killed.
    pub kill_sig_rx: watch::Receiver<bool>,
}

pub struct SessionServiceFactory { }

impl SessionServiceFactory {
    pub fn new() -> Self {
        SessionServiceFactory {}
    }

    pub fn make(&self) -> Arc<Service> {
        Arc::new(Service::new())
    }
}

pub struct Service {
    items: DashMap<String, Arc<Session>>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            items: DashMap::new(),
        }
    }

    pub fn add_session(&self, session: Arc<Session>) {
        self.items.insert(session.ticket.clone(), session);
    }

    pub fn has(&self, ticket: &String) -> bool {
        self.items.contains_key(ticket)
    }

    pub fn delete(&self, ticket: &String) {
        self.items.remove(ticket);
    }

    pub fn online_bots(&self) -> usize {
        return self.items.len();
    }

    pub fn all(&self) -> Vec<Arc<Session>> {
        return self.items.iter().map(|item| item.value().clone()).collect();
    }

    pub async fn broadcast(&self, msg: Message) {
        for entry in self.items.iter() {
            entry.value().packet_tx.send(msg.clone()).await.unwrap_or_else(|error| {
                error!("unable to send packet to the server: {:?}", error);
            });
        }
    }

    pub async fn send(&self, session: &Arc<Session>, msg: Message) -> anyhow::Result<()> {
        if !self.items.contains_key(&session.ticket) {
            return Err(anyhow!("Ticket does not exist"));
        }

        session.packet_tx.send(msg.clone()).await?;

        Ok(())
    }
}