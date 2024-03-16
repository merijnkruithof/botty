use std::{fmt, sync::Arc};

use tokio::sync::{mpsc::Sender};
use tokio_tungstenite::tungstenite::Message;
use anyhow::{anyhow, Result};
use dashmap::DashMap;
use tracing::error;

#[derive(Debug)]
pub struct SessionError {
    details: String,
}

impl SessionError {
    fn new(msg: &str) -> SessionError {
        SessionError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

pub struct Session {
    pub ticket: String,
    pub tx: Sender<Message>,
    pub kill_sig_rx: tokio::sync::watch::Receiver<bool>,
    pub kill_sig_tx: tokio::sync::watch::Sender<bool>,
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

    pub fn kill(&self, auth_ticket: String) -> Result<()> {
        let session = self.items
            .get(&auth_ticket)
            .ok_or_else(|| anyhow!("No session found for auth ticket {}", &auth_ticket))?;

        session.kill_sig_tx.send(true)?;

        Ok(())
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

    pub async fn broadcast(&self, msg: Message) {
        for entry in self.items.iter() {
            entry.value().tx.send(msg.clone()).await.unwrap_or_else(|error| {
                error!("unable to send packet to the server: {:?}", error);
            });
        }
    }

    pub async fn send(&self, session: &Session, msg: &Message) -> Result<()> {
        if !self.items.contains_key(&session.ticket) {
            return Err(anyhow!("Ticket does not exist"));
        }

        session.tx.send(msg.clone()).await?;

        Ok(())
    }
}
