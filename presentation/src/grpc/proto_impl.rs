use todo::todo_service_server::TodoService;
use todo::{GetTodosRequest, GetTodosResponse, Todo};
use use_case::traits::todo::TodoUseCase;

pub mod todo {
    tonic::include_proto!("todo");
}

#[derive(Default)]
pub struct TodoServiceImpl<TU: TodoUseCase> {
    tu: TU,
}

#[tonic::async_trait]
impl<TU: TodoUseCase> TodoService for TodoServiceImpl<TU> {
    async fn get_todos(
        &self,
        _request: tonic::Request<GetTodosRequest>,
    ) -> Result<tonic::Response<GetTodosResponse>, tonic::Status> {
        let todos = self.tu.find_all().await;
        match todos {
            Ok(todos) => {
                let todos: Vec<Todo> = todos
                    .into_iter()
                    .map(|todo| Todo {
                        id: todo.id,
                        title: todo.title.unwrap_or("".to_string()),
                    })
                    .collect();
                let response = GetTodosResponse { todos };
                return Ok(tonic::Response::new(response));
            }
            Err(_) => {
                return Err(tonic::Status::internal("Internal Server Error".to_string()));
            }
        }
    }
}
