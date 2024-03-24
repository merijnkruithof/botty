use std::sync::Arc;
use anyhow::anyhow;
use dashmap::DashMap;
use crate::client;
use crate::client::session;

// Manager takes full responsibility of listing, adding, and removing retro's from Pegasus. It
// stores a hotel name, e.g. "Localhost", and couples it to an underlying client hotel manager.
//
// The client::hotel::Manager is the entrypoint of each added hotel and is responsible for traffic.
pub struct Manager {
    handlers: DashMap<String, Arc<client::hotel::Manager>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            handlers: DashMap::new(),
        }
    }

    pub async fn add_hotel(&self, name: String, hotel_manager: Arc<client::hotel::Manager>) -> anyhow::Result<()> {
        if self.handlers.contains_key(&name) {
            return Err(anyhow!("Handler {} already exists", name));
        }

        self.handlers.insert(name.clone(), hotel_manager);

        Ok(())
    }

    pub fn get_hotel_connection_handler(&self, name: String) -> anyhow::Result<Arc<client::hotel::Manager>> {
        if !self.handlers.contains_key(&name) {
            return Err(anyhow!("Unable to find handler with name {}", &name));
        }

        let handler = self.handlers.get(&name).unwrap();

        Ok(handler.clone())
    }

    pub fn list_retros(&self) -> anyhow::Result<Vec<String>> {
        return Ok(self.handlers
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
        );
    }
}