use std::sync::Arc;
use tokio::sync::{broadcast, watch};

use crate::event::ControllerEvent;
use anyhow::{Result};
use dashmap::DashMap;
use tracing::{debug, info};
use crate::connection::session::Session;
use crate::user::user::User;

pub struct Manager {
    users: Arc<DashMap<String, Arc<User>>>
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            users: Arc::new(DashMap::new())
        }
    }

    pub fn users(&self) -> Vec<Arc<User>> {
        self.users
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn get_user(&self, auth_ticket: String) -> Option<Arc<User>> {
        if let Some(entry) = self.users.get(&auth_ticket) {
            Some(entry.value().clone())
        } else {
            None
        }
    }

    pub async fn listen(&self,
                        session: Arc<Session>,
                        event_handler_rx: broadcast::Receiver<ControllerEvent>,
                        mut kill_rx: watch::Receiver<bool>
    ) -> Result<()> {
        info!("Started user listener");

        let mut subscriber = event_handler_rx.resubscribe();
        loop {
            tokio::select! {
                Ok(data) = subscriber.recv() => self.handle_controller_event(session.clone(), data),
                Ok(_) = kill_rx.changed() => {
                    if *kill_rx.borrow() {
                        break;
                    }
                }
            }
        }

        if let Some(_) = self.users.remove(&session.ticket) {
            info!("Removed user with auth ticket {}", &session.ticket);
        }

        info!("Stopped user listener.");

        Ok(())
    }

    fn handle_controller_event(&self, session: Arc<Session>, event: ControllerEvent) {
        let users_map = self.users.clone();

        tokio::spawn(async move {
            match event {
                ControllerEvent::UserInfo { data } => {
                    debug!("Received user data {:?}", &data);

                    users_map.insert(session.ticket.clone(), Arc::new(User{
                        user_id: data.user_id,
                        username: data.username,
                        motto: data.motto,
                        figure: data.figure,
                        gender: data.gender,
                        sso_ticket: session.ticket.clone()
                    }));

                    info!("Updated user data for session {}", &session.ticket);
                },
                _ => {}
            }
        });
    }
}