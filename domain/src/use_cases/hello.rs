//! The greeting use case: build the greeting message for a requested name.

use serde::{Deserialize, Serialize};

use crate::errors::GreeterError;

/// The minimum length (in bytes) of an accepted name.
pub const MIN_NAME_LENGTH: usize = 3;

/// Request of [`HelloUseCase`]: the name to greet.
#[derive(Debug, Deserialize)]
pub struct HelloRequest {
    pub name: String,
}

/// Response of [`HelloUseCase`]: the greeting message.
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloResponse {
    pub message: String,
}

/// Build a greeting message for the requested name.
#[derive(Debug, Default)]
pub struct HelloUseCase;

impl HelloUseCase {
    /// Construct the use case.
    pub fn new() -> Self {
        HelloUseCase
    }

    /// Return the greeting message for the requested name.
    pub fn execute(&self, request: HelloRequest) -> Result<HelloResponse, GreeterError> {
        if request.name.len() < MIN_NAME_LENGTH {
            return Err(GreeterError::InvalidNameLength);
        }

        Ok(HelloResponse {
            message: format!("Hello, {}!", request.name),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greet(name: &str) -> Result<String, GreeterError> {
        HelloUseCase::new()
            .execute(HelloRequest {
                name: name.to_string(),
            })
            .map(|response| response.message)
    }

    #[test]
    fn greets_the_requested_name() {
        assert_eq!(greet("Alice").unwrap(), "Hello, Alice!");
    }

    #[test]
    fn accepts_a_name_of_exactly_the_minimum_length() {
        assert_eq!(greet("abc").unwrap(), "Hello, abc!");
    }

    #[test]
    fn rejects_a_name_shorter_than_the_minimum_length() {
        assert!(matches!(greet("ab"), Err(GreeterError::InvalidNameLength)));
    }

    #[test]
    fn measures_the_name_in_bytes_not_in_characters() {
        // "éé" is two characters but four bytes: the byte length is what matters.
        assert_eq!(greet("éé").unwrap(), "Hello, éé!");
    }
}
