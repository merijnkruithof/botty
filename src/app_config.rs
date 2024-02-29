use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub uri: String,
    pub tickets: Vec<String>,
}

pub fn load() -> Result<ServerConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("config")).unwrap();

    cfg.try_into::<ServerConfig>()
}
