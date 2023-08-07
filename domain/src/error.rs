pub enum DomainError {
    Validation(String),
    NotFound { entity_type: String, entity_id: i32 },
    Infrastructure(anyhow::Error),
    Unexpected(String),
}
