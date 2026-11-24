use std::env;
use std::net::IpAddr;
use tracing::error;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub address: IpAddr,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            address: env::var("ADDRESS")
                .ok()
                .and_then(|a| a.parse().ok())
                .unwrap_or_else(|| "0.0.0.0".parse().unwrap()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|e| {
                    error!("error retrieving database URL: {e}");
                    panic!("failed to retrieve database URL");
                }),
        }
    }
}
