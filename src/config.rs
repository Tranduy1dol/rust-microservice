use std::sync::LazyLock;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,

    #[clap(short, long, default_value = "3030")]
    pub port: u16,

    #[clap(long, default_value = "postgres")]
    pub database_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub enable_query_log: bool,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            enable_query_log: true,
            max_connections: 10,
            min_connections: 1,
            connect_timeout: 8,
            acquire_timeout: 8,
            idle_timeout: 300,
            max_lifetime: 1800,
        }
    }
}

pub static APP_CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);
