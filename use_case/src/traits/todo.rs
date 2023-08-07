use async_trait::async_trait;

use crate::{
    dto::todo::{CreateTodoDto, TodoDto},
    error::UseCaseError,
};

#[async_trait]
pub trait CreateTodoUseCase: Send + Sync + 'static {
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError>;
}
