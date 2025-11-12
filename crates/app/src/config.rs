use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    #[allow(dead_code)]
    pub redis: RedisConfig,
    #[allow(dead_code)]
    pub jwt: JwtConfig,
    #[allow(dead_code)]
    pub log: LogConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    #[allow(dead_code)]
    pub allowed_origin: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    #[allow(dead_code)]
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    #[allow(dead_code)]
    pub secret: String,
    #[allow(dead_code)]
    pub expires_in_hours: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    #[allow(dead_code)]
    pub level: String,
}

impl Config {
    /// Load application configuration by merging default and production TOML files and environment variables.
    ///
    /// This initializes dotenv, then merges `config/default.toml`, `config/production.toml` (as nested),
    /// and environment variables prefixed with `APP_` (split by `__`) into a `Config` value.
    ///
    /// # Returns
    ///
    /// `Ok(Config)` with the merged configuration on success, `Err(figment::Error)` if loading or deserialization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// // Load configuration and panic on failure in examples/tests
    /// let cfg = crate::config::Config::new().unwrap();
    /// // access a field to illustrate usage
    /// let _port = cfg.server.port;
    /// ```
    #[allow(clippy::result_large_err)]
    pub fn new() -> Result<Self, figment::Error> {
        dotenv::dotenv().ok();

        Figment::new()
            .merge(Toml::file("config/default.toml"))
            .merge(Toml::file("config/production.toml").nested())
            .merge(Env::prefixed("APP_").split("__"))
            // Trích xuất config
            .extract()
    }
}

impl Default for Config {
    /// Creates a `Config` by loading configuration from the default TOML, optional production TOML, and environment variables; panics if loading fails.
    ///
    /// # Examples
    ///
    /// ```
    /// // Requires `Config` to be in scope:
    /// let cfg = Config::default();
    /// // Use the loaded configuration
    /// let _port = cfg.server.port;
    /// ```
    fn default() -> Self {
        Self::new().expect("Failed to load configuration")
    }
}
