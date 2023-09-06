use async_trait::async_trait;

use crate::{
    dto::todo::{CreateTodoDto, TodoDto},
    error::UseCaseError,
};

#[async_trait]
pub trait MutationUseCase: Send + Sync + 'static {
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError>;
    async fn update(&self, todo_data: TodoDto) -> Result<TodoDto, UseCaseError>;
    async fn delete(&self, todo_id: i64) -> Result<i64, UseCaseError>;
}

#[async_trait]
pub trait QueryUseCase: Send + Sync + 'static {
    async fn find_all(&self) -> Result<Vec<TodoDto>, UseCaseError>;
    async fn find_by_id(&self, todo_id: i64) -> Result<Option<TodoDto>, UseCaseError>;
}

#[async_trait]
pub trait TodoUseCase: Send + Sync + 'static {
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError>;
    async fn update(&self, todo_data: TodoDto) -> Result<TodoDto, UseCaseError>;
    async fn delete(&self, todo_id: i64) -> Result<i64, UseCaseError>;
    async fn find_all(&self) -> Result<Vec<TodoDto>, UseCaseError>;
    async fn find_by_id(&self, todo_id: i64) -> Result<Option<TodoDto>, UseCaseError>;
}
