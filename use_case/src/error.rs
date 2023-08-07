use domain::error::DomainError;

pub enum UseCaseError {
    Validation(String),
    NotFound { entity_type: String, entity_id: i32 },
    Other(anyhow::Error),
    Unexpected(String),
}

impl From<DomainError> for UseCaseError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::Validation(message) => Self::Validation(message),
            DomainError::NotFound {
                entity_type,
                entity_id,
            } => Self::NotFound {
                entity_type,
                entity_id,
            },
            DomainError::Infrastructure(error) => Self::Other(error),
            DomainError::Unexpected(message) => Self::Unexpected(message),
        }
    }
}
