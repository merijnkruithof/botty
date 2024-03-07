use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
};

use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

// Define your error types
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

    pub fn insert(&mut self, session: Arc<Session>) {
        let mut write_lock = self.items.write().unwrap();

        write_lock.insert(session.ticket.clone(), session);
    }

    pub fn delete(&mut self, ticket: &String) {
        let mut write_lock = self.items.write().unwrap();

        write_lock.remove(ticket);
    }

    pub fn online_bots(&self) -> usize {
        let read_lock = self.items.read().unwrap();

        read_lock.values().count()
    }

    pub async fn broadcast<F>(&self, msg: Message) {
        let read_lock = self.items.read().unwrap();

        for (_, session) in read_lock.iter() {
            session
                .tx
                .send(msg.clone())
                .await
                .expect("unable to send message to channel");
        }
    }

    pub async fn send(&self, session: &Session, msg: &Message) -> Result<(), SessionError> {
        let read_lock = self.items.read().unwrap();

        if !read_lock.contains_key(&session.ticket) {
            return Err(SessionError::new("Session ticket not found in items"));
        }

        session
            .tx
            .send(msg.clone())
            .await
            .expect("unable to send message to channel");

        Ok(())
    }
}
