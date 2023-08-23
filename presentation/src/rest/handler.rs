use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use use_case::{error::UseCaseError, traits::todo::QueryUseCase};

use super::object::Todo;

pub async fn get_todos<QU: QueryUseCase>(State(qu): State<QU>) -> impl IntoResponse {
    let todos = qu.find_all().await;
    if let Err(err) = todos {
        let res = match err {
            UseCaseError::NotFound {
                entity_id: _,
                entity_type: _,
            } => (StatusCode::NOT_FOUND, Json(vec![])),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
        };
        return res;
    }
    if let Ok(todos) = todos {
        let todos = todos
            .into_iter()
            .map(|todo| todo.into())
            .collect::<Vec<Todo>>();
        return (StatusCode::OK, Json(todos));
    }
    (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
}
