//! Application configuration read from the environment (and from `.env` when present).
//!
//! Every field can be set by the environment variable of the same name; the defaults are the
//! addresses the application has always served on: HTTP on `0.0.0.0:3000`, gRPC on
//! `0.0.0.0:50051`.

use std::net::SocketAddr;
use std::sync::OnceLock;

use dotenv::dotenv;

/// Application settings; every field is set by the environment variable of the same name.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// The host the HTTP server binds to.
    pub http_host: String,
    /// The port the HTTP server listens on.
    pub http_port: u16,
    /// The host the gRPC server binds to.
    pub grpc_host: String,
    /// The port the gRPC server listens on.
    pub grpc_port: u16,
    /// Logging severity of the application code and the request log (when `RUST_LOG` is unset).
    pub log_level: String,
}

impl AppConfig {
    /// Read the configuration from the process environment.
    fn from_env() -> Self {
        dotenv().ok();

        AppConfig {
            http_host: std::env::var("HTTP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            http_port: std::env::var("HTTP_PORT")
                .ok()
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(3000),
            grpc_host: std::env::var("GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            grpc_port: std::env::var("GRPC_PORT")
                .ok()
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(50051),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string()),
        }
    }

    /// The address the HTTP server binds to.
    pub fn http_address(&self) -> SocketAddr {
        parse_address(&self.http_host, self.http_port)
    }

    /// The address the gRPC server binds to.
    pub fn grpc_address(&self) -> SocketAddr {
        parse_address(&self.grpc_host, self.grpc_port)
    }
}

fn parse_address(host: &str, port: u16) -> SocketAddr {
    format!("{host}:{port}")
        .parse()
        .expect("invalid server address in the configuration")
}

/// Return the settings, read once per process.
pub fn get_config() -> &'static AppConfig {
    static CONFIG: OnceLock<AppConfig> = OnceLock::new();
    CONFIG.get_or_init(AppConfig::from_env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_settings_from_the_environment_and_applies_the_defaults() {
        std::env::set_var("HTTP_HOST", "127.0.0.1");
        std::env::set_var("HTTP_PORT", "8080");
        std::env::set_var("GRPC_HOST", "127.0.0.1");
        std::env::set_var("GRPC_PORT", "50052");
        std::env::set_var("LOG_LEVEL", "debug");

        let config = AppConfig::from_env();
        assert_eq!(config.http_address().to_string(), "127.0.0.1:8080");
        assert_eq!(config.grpc_address().to_string(), "127.0.0.1:50052");
        assert_eq!(config.log_level, "debug");

        std::env::remove_var("HTTP_HOST");
        std::env::remove_var("HTTP_PORT");
        std::env::remove_var("GRPC_HOST");
        std::env::remove_var("GRPC_PORT");
        std::env::remove_var("LOG_LEVEL");

        let config = AppConfig::from_env();
        assert_eq!(config.http_address().to_string(), "0.0.0.0:3000");
        assert_eq!(config.grpc_address().to_string(), "0.0.0.0:50051");
        assert_eq!(config.log_level, "INFO");
    }
}
