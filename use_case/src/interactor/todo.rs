use async_trait::async_trait;
use domain::{entity::todo::Todo, repository::todo_repository::TodoRepository};

use crate::{
    dto::todo::{CreateTodoDto, TodoDto},
    error::UseCaseError,
    traits::todo::CreateTodoUseCase,
};

pub struct CreateTodoInteractor<TR> {
    todo_repository: TR,
}

impl<TR> CreateTodoInteractor<TR> {
    pub fn new(todo_repository: TR) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl<TR> CreateTodoUseCase for CreateTodoInteractor<TR>
where
    TR: TodoRepository,
{
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError> {
        let todo = Todo::try_from(todo_data)?;
        self.todo_repository.create(&todo).await?;
        Ok(todo.into())
    }
}
