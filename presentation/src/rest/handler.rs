use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use use_case::{error::UseCaseError, traits::todo::TodoUseCase};

use crate::error::PresentationalError;

use super::object::{
    CreateTodoPayload, CreateTodoResponse, DeleteTodoPayload, DeleteTodoResponse, Todo,
    TodoResponse, TodosResponse, UpdateTodoPayload, UpdateTodoResponse,
};

pub async fn get_todos<TU: TodoUseCase>(Extension(tu): Extension<TU>) -> impl IntoResponse {
    let todos = tu.find_all().await;
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

pub async fn get_todo<TU: TodoUseCase>(
    Extension(tu): Extension<TU>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let todo = tu.find_by_id(id).await;
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

pub async fn create_todo<TU: TodoUseCase>(
    Extension(tu): Extension<TU>,
    Json(payload): Json<CreateTodoPayload>,
) -> impl IntoResponse {
    let todo = tu.create(payload.into()).await;
    if todo.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateTodoResponse {
                todo: None,
                error: Some(PresentationalError::InternalServerError),
            }),
        );
    }
    if let Ok(todo) = todo {
        let todo: Todo = todo.into();
        return (
            StatusCode::OK,
            Json(CreateTodoResponse {
                todo: Some(todo),
                error: None,
            }),
        );
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(CreateTodoResponse {
            todo: None,
            error: Some(PresentationalError::InternalServerError),
        }),
    )
}

pub async fn update_todo<TU: TodoUseCase>(
    Extension(tu): Extension<TU>,
    Json(payload): Json<UpdateTodoPayload>,
) -> impl IntoResponse {
    let todo = tu.update(payload.into()).await;
    if todo.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UpdateTodoResponse {
                todo: None,
                error: Some(PresentationalError::InternalServerError),
            }),
        );
    }
    if let Ok(todo) = todo {
        let todo: Todo = todo.into();
        return (
            StatusCode::OK,
            Json(UpdateTodoResponse {
                todo: Some(todo),
                error: None,
            }),
        );
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(UpdateTodoResponse {
            todo: None,
            error: Some(PresentationalError::InternalServerError),
        }),
    )
}

pub async fn delete_todo<TU: TodoUseCase>(
    Extension(tu): Extension<TU>,
    Json(payload): Json<DeleteTodoPayload>,
) -> impl IntoResponse {
    let todo_id = payload.id;

    // check if todo exists
    let find_todo_result = tu.find_by_id(todo_id).await;
    let todo = match find_todo_result {
        Ok(todo) => todo,
        Err(_) => None,
    };

    if todo.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(DeleteTodoResponse {
                todo: None,
                error: Some(PresentationalError::NotFound),
            }),
        );
    }
    let todo = todo.unwrap();

    let delete_todo_result = tu.delete(payload.id).await;
    if delete_todo_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DeleteTodoResponse {
                todo: None,
                error: Some(PresentationalError::InternalServerError),
            }),
        );
    }
    if delete_todo_result.is_ok() {
        let todo: Todo = todo.into();
        return (
            StatusCode::OK,
            Json(DeleteTodoResponse {
                todo: Some(todo),
                error: None,
            }),
        );
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(DeleteTodoResponse {
            todo: None,
            error: Some(PresentationalError::InternalServerError),
        }),
    )
}
