//! Use cases: one module per business operation, each exposing its operation object and its
//! request and response models.
//!
//! The use cases are the application layer: they orchestrate the business operations and are
//! independent of any transport.

pub mod hello;

pub use hello::{HelloRequest, HelloResponse, HelloUseCase};
