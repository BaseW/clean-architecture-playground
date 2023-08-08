use domain::entity::todo::Todo;

use crate::error::UseCaseError;

#[derive(Debug, Clone)]
pub struct TodoDto {
    pub id: i64,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateTodoDto {
    pub title: String,
}

impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
        }
    }
}

impl TryFrom<CreateTodoDto> for Todo {
    type Error = UseCaseError;

    fn try_from(todo_data: CreateTodoDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: 0,
            title: Some(todo_data.title),
        })
    }
}
