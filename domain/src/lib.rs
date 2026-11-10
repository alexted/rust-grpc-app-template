//! Domain layer: the application use cases with their request and response models, and the
//! shared error vocabulary.
//!
//! This crate is the innermost layer of the application. It depends on nothing but the data
//! (de)serialization contracts; the transports and the infrastructure depend on it, never the
//! other way around.

pub mod errors;
pub mod use_cases;
