use thiserror::Error;

#[derive(Debug, Error)]
pub enum GreeterError {
    #[error("Invalid name length")]
    InvalidNameLength,
}

pub type GreeterResult<T> = Result<T, GreeterError>;

pub struct HelloInput {
    pub name: String,
}

pub struct HelloOutput {
    pub message: String,
}

pub fn hello(input: HelloInput) -> GreeterResult<HelloOutput> {
    let HelloInput { name } = input;
    if name.len() < 3 {
        return Err(GreeterError::InvalidNameLength);
    }

    let message = format!("Hello, {name}!");
    Ok(HelloOutput { message })
}
