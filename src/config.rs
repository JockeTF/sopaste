use std::fmt::Display;
use std::net::SocketAddr;
use std::str::FromStr;

use sqlx::mysql::MySqlConnectOptions;

fn env<T>(key: &str) -> Result<T, String>
where
    T: FromStr,
    T::Err: Display,
{
    std::env::var(key)
        .map_err(|e| format!("{key}: {e}"))?
        .parse()
        .map_err(|e| format!("{key}: {e}"))
}

pub struct Config {
    pub binding: SocketAddr,
    pub database: MySqlConnectOptions,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            binding: env("SOPASTE_BINDING")?,
            database: env("SOPASTE_DATABASE")?,
        })
    }
}
