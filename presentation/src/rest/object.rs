use serde::{Deserialize, Serialize};
use use_case::dto::todo::TodoDto;

use crate::error::PresentationalError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: Option<String>,
}

impl From<TodoDto> for Todo {
    fn from(todo_dto: TodoDto) -> Self {
        Self {
            id: todo_dto.id,
            title: todo_dto.title,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodosResponse {
    pub todos: Option<Vec<Todo>>,
    pub error: Option<PresentationalError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoResponse {
    pub todo: Option<Todo>,
    pub error: Option<PresentationalError>,
}
