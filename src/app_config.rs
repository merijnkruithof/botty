use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Handler {
    pub name: String,
    pub ws_link: String,
    pub origin: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub webserver: String,
    pub auth_token: String,
    pub use_default_handlers: bool,
    pub handlers: Vec<Handler>
}

pub fn load() -> Result<ServerConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("config")).unwrap();

    cfg.try_into::<ServerConfig>()
}
