use serde::Deserialize;
use config::ConfigError;
use config;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}