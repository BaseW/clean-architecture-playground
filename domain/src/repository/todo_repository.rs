use async_trait::async_trait;

use crate::{entity::todo::Todo, error::DomainError};

#[async_trait]
pub trait TodoRepository: Send + Sync + 'static {
    async fn create(&self, todo: &Todo) -> Result<(), DomainError>;
    // async fn find_all(&self) -> Result<Vec<Todo>, DomainError>;
    // async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, DomainError>;
    // async fn update(&self, id: i32, todo: &Todo) -> Result<Todo, DomainError>;
    // async fn delete(&self, todo_id: i32) -> Result<(), DomainError>;
}