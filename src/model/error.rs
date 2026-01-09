use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Data Model Parse error: {0}")]
    Parse(String),
    #[error("Data Model Format error: {0}")]
    Format(String),
    #[error("Data Model Validation error: {0}")]
    Validation(String),
}
