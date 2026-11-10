//! Composition root of the application dependencies.
//!
//! The container constructs the application components once per process and shares them with
//! every transport adapter: the HTTP and the gRPC servers both take it as their state, and the
//! handlers run the use cases through it.
//!
//! When the application gains data sources (repositories, clients), they are created here and
//! the use cases are injected with them — this is the single place where the object graph is
//! wired.

use std::sync::Arc;

use my_service_domain::use_cases::HelloUseCase;

/// The application dependencies, constructed once and shared by the servers.
pub struct Container {
    /// The greeting operation, shared by the HTTP and the gRPC adapters.
    pub hello_use_case: Arc<HelloUseCase>,
}

impl Container {
    /// Build the application dependencies.
    pub fn new() -> Self {
        Container {
            hello_use_case: Arc::new(HelloUseCase::new()),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
