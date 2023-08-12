use async_trait::async_trait;
use domain::{entity::todo::Todo, repository::todo_repository::TodoRepository};

use crate::{
    dto::todo::{CreateTodoDto, TodoDto},
    error::UseCaseError,
    traits::todo::{MutationUseCase, QueryUseCase},
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
impl<TR> MutationUseCase for MutationTodoInteractor<TR>
where
    TR: TodoRepository,
{
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError> {
        let todo = Todo::try_from(todo_data)?;
        self.todo_repository.create(&todo).await?;
        Ok(todo.into())
    }

    async fn delete(&self, todo_id: i64) -> Result<i64, UseCaseError> {
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
impl<TR> QueryUseCase for QueryInteractor<TR>
where
    TR: TodoRepository,
{
    async fn find_all(&self) -> Result<Vec<TodoDto>, UseCaseError> {
        let result = self.todo_repository.find_all().await;
        match result {
            Ok(todos) => {
                let todo_dtos = todos.into_iter().map(|todo| todo.into()).collect();
                Ok(todo_dtos)
            }
            Err(e) => Err(UseCaseError::from(e)),
        }
    }
}
