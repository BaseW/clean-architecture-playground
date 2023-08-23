use serde::{Deserialize, Serialize};
use use_case::dto::todo::TodoDto;

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
