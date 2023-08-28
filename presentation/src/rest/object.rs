use serde::{Deserialize, Serialize};
use use_case::dto::todo::{CreateTodoDto, TodoDto};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoPayload {
    pub title: String,
}

impl From<CreateTodoPayload> for CreateTodoDto {
    fn from(create_todo_payload: CreateTodoPayload) -> Self {
        Self {
            title: create_todo_payload.title,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoPayload {
    pub id: i64,
    pub title: String,
}

impl From<UpdateTodoPayload> for TodoDto {
    fn from(update_todo_payload: UpdateTodoPayload) -> Self {
        Self {
            id: update_todo_payload.id,
            title: Some(update_todo_payload.title),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoResponse {
    pub todo: Option<Todo>,
    pub error: Option<PresentationalError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoResponse {
    pub todo: Option<Todo>,
    pub error: Option<PresentationalError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTodoResponse {
    pub todo: Option<Todo>,
    pub error: Option<PresentationalError>,
}
