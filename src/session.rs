use std::{collections::HashMap, fmt, sync::Arc};
use std::sync::atomic::AtomicBool;

use tokio::sync::{mpsc::Sender, RwLock};
use tokio_tungstenite::tungstenite::Message;

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
    items: RwLock<HashMap<String, Arc<Session>>>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            items: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_session(&mut self, session: Arc<Session>) {
        let mut write_lock = self.items.write().await;

        write_lock.insert(session.ticket.clone(), session);
    }

    pub async fn delete(&mut self, ticket: &String) {
        let mut write_lock = self.items.write().await;

        write_lock.remove(ticket);
    }

    pub async fn online_bots(&self) -> usize {
        let read_lock = self.items.read().await;

        read_lock.values().count()
    }

    pub async fn broadcast(&self, msg: Message) {
        let read_lock = self.items.read().await;

        for (_, session) in read_lock.iter() {
            session.tx.send(msg.clone()).await.unwrap_or_else(|error| {
                eprintln!("unable to send packet to the server: {:?}", error);
            });
        }
    }

    pub async fn send(&self, session: &Session, msg: &Message) -> Result<(), SessionError> {
        let read_lock = self.items.read().await;

        if !read_lock.contains_key(&session.ticket) {
            return Err(SessionError::new("Session ticket not found in items"));
        }

        session.tx.send(msg.clone()).await.unwrap_or_else(|error| {
            eprintln!("unable to send packet to the server: {:?}", error);
        });

        Ok(())
    }
}
