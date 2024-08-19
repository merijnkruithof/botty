use std::sync::Arc;
use anyhow::anyhow;
use dashmap::DashMap;
use tokio::sync::{mpsc::Sender};
use tokio_tungstenite::tungstenite::Message;
use tracing::error;

pub struct Session {
    // ticket contains the authentication ticket of a session. This is currently its unique
    // identifier.
    pub ticket: String,

    // packet_tx is a channel for sending packets to the underlying network stream.
    pub packet_tx: Sender<Message>,
}

impl Session {
    pub fn new(ticket: String, packet_tx: Sender<Message>) -> Self {
        Session { ticket, packet_tx }
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

    pub fn get(&self, ticket: &String) -> Option<Arc<Session>> {
        if let Some(session)  = self.items.get(ticket) {
            Some(session.clone())
        } else {
            None
        }
    }

    pub fn delete(&self, ticket: &String) {
        self.items.remove(ticket);
    }

    pub fn online_bots(&self) -> usize {
        self.items.len()
    }

    pub fn all(&self) -> Vec<Arc<Session>> {
        self.items.iter().map(|item| item.value().clone()).collect()
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