use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use use_case::{error::UseCaseError, traits::todo::QueryUseCase};

use crate::error::PresentationalError;

use super::object::{Todo, TodoResponse, TodosResponse};

pub async fn get_todos<QU: QueryUseCase>(State(qu): State<QU>) -> impl IntoResponse {
    let todos = qu.find_all().await;
    if let Err(err) = todos {
        let res = match err {
            UseCaseError::NotFound {
                entity_id: _,
                entity_type: _,
            } => (
                StatusCode::NOT_FOUND,
                Json(TodosResponse {
                    todos: None,
                    error: Some(PresentationalError::NotFound),
                }),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TodosResponse {
                    todos: None,
                    error: Some(PresentationalError::InternalServerError),
                }),
            ),
        };
        return res;
    }
    if let Ok(todos) = todos {
        let todos = todos
            .into_iter()
            .map(|todo| todo.into())
            .collect::<Vec<Todo>>();
        return (
            StatusCode::OK,
            Json(TodosResponse {
                todos: Some(todos),
                error: None,
            }),
        );
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(TodosResponse {
            todos: None,
            error: Some(PresentationalError::InternalServerError),
        }),
    )
}

pub async fn get_todo<QU: QueryUseCase>(
    Path(id): Path<i64>,
    State(qu): State<QU>,
) -> impl IntoResponse {
    let todo = qu.find_by_id(id).await;
    if let Err(err) = todo {
        let res = match err {
            UseCaseError::NotFound {
                entity_id: _,
                entity_type: _,
            } => (
                StatusCode::NOT_FOUND,
                Json(TodoResponse {
                    todo: None,
                    error: Some(PresentationalError::NotFound),
                }),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TodoResponse {
                    todo: None,
                    error: Some(PresentationalError::InternalServerError),
                }),
            ),
        };
        return res;
    }
    if let Ok(todo) = todo {
        if let Some(todo) = todo {
            let todo: Todo = todo.into();
            return (
                StatusCode::OK,
                Json(TodoResponse {
                    todo: Some(todo),
                    error: None,
                }),
            );
        } else {
            return (
                StatusCode::OK,
                Json(TodoResponse {
                    todo: None,
                    error: Some(PresentationalError::NotFound),
                }),
            );
        }
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(TodoResponse {
            todo: None,
            error: Some(PresentationalError::InternalServerError),
        }),
    )
}
