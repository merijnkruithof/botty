use std::{
    collections::{HashMap, HashSet},
    fmt,
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
    items: HashMap<String, Session>,
}

impl Service {
    pub fn new(items: HashMap<String, Session>) -> Service {
        Service { items }
    }

    pub fn insert(&mut self, session: Session) {
        self.items.insert(session.ticket.clone(), session);
    }

    pub fn delete(&mut self, ticket: &String) {
        self.items.remove(ticket);
    }

    pub async fn broadcast(&self, msg: &Message) {
        for (_, session) in &self.items {
            session
                .tx
                .send(msg.clone())
                .await
                .expect("unable to send message to channel");
        }
    }

    pub fn all(&self) -> Vec<&Session> {
        let mut sessions: Vec<&Session> = Vec::new();

        for (_, session) in &self.items {
            sessions.push(session);
        }

        sessions
    }

    pub async fn send(&self, session: &Session, msg: &Message) -> Result<(), SessionError> {
        if !self.items.contains_key(&session.ticket) {
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
