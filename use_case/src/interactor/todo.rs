use async_trait::async_trait;
use domain::{entity::todo::Todo, repository::todo_repository::TodoRepository};

use crate::{
    dto::todo::{CreateTodoDto, TodoDto},
    error::UseCaseError,
    traits::todo::{MutationTodoUseCase, QueryTodoUseCase},
};

pub struct MutationTodoInteractor<TR> {
    todo_repository: TR,
}

impl<TR> MutationTodoInteractor<TR> {
    pub fn new(todo_repository: TR) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl<TR> MutationTodoUseCase for MutationTodoInteractor<TR>
where
    TR: TodoRepository,
{
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError> {
        let todo = Todo::try_from(todo_data)?;
        self.todo_repository.create(&todo).await?;
        Ok(todo.into())
    }

    async fn delete(&self, todo_id: i32) -> Result<i32, UseCaseError> {
        self.todo_repository.delete(todo_id).await?;
        Ok(todo_id)
    }
}

pub struct QueryInteractor<TR> {
    todo_repository: TR,
}

impl<TR> QueryInteractor<TR> {
    pub fn new(todo_repository: TR) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl<TR> QueryTodoUseCase for QueryInteractor<TR>
where
    TR: TodoRepository,
{
    async fn find_all(&self) -> Result<Vec<Todo>, UseCaseError> {
        let result = self.todo_repository.find_all().await;
        match result {
            Ok(todos) => Ok(todos),
            Err(e) => Err(UseCaseError::from(e)),
        }
    }
}
