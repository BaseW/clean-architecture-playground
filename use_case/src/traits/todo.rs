use async_trait::async_trait;
use domain::entity::todo::Todo;

use crate::{
    dto::todo::{CreateTodoDto, TodoDto},
    error::UseCaseError,
};

#[async_trait]
pub trait MutationTodoUseCase: Send + Sync + 'static {
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError>;
    async fn delete(&self, todo_id: i32) -> Result<i32, UseCaseError>;
}

#[async_trait]
pub trait QueryTodoUseCase: Send + Sync + 'static {
    async fn find_all(&self) -> Result<Vec<Todo>, UseCaseError>;
}
