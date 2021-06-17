pub use config::ConfigError;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Configuration {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;

        return cfg.try_into();
    }
}