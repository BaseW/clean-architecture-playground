use serde::{Deserialize, Serialize};
use std::fmt::Display;
use use_case::error::UseCaseError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PresentationalError {
    NotFound,
    InternalServerError,
}

impl From<UseCaseError> for PresentationalError {
    fn from(error: UseCaseError) -> Self {
        match error {
            UseCaseError::NotFound {
                entity_type: _,
                entity_id: _,
            } => Self::NotFound,
            _ => Self::InternalServerError,
        }
    }
}

impl Display for PresentationalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresentationalError::NotFound => write!(f, "Not Found"),
            PresentationalError::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}
