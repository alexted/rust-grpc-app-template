//! Application error vocabulary shared by the use cases and the transport adapters.
//!
//! The errors are defined independently of HTTP and gRPC: each transport maps them to its own
//! protocol representation at its boundary (HTTP status codes, gRPC statuses).

use thiserror::Error;

/// Errors of the greeting operations.
#[derive(Debug, Error)]
pub enum GreeterError {
    /// The name is shorter than the minimum length.
    #[error("Invalid name length")]
    InvalidNameLength,
}
