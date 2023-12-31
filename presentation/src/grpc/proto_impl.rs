use todo::todo_service_server::TodoService;
pub use todo::{
    CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse,
    FindTodoByIdRequest, FindTodoByIdResponse, GetTodosRequest, GetTodosResponse, Todo,
    UpdateTodoRequest, UpdateTodoResponse,
};
use use_case::{
    dto::todo::{CreateTodoDto, TodoDto},
    traits::todo::TodoUseCase,
};

pub use todo::todo_service_client::TodoServiceClient;
pub use todo::todo_service_server::TodoServiceServer;

pub mod todo {
    tonic::include_proto!("todo");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("todo_descriptor");
}

#[derive(Default)]
pub struct TodoServiceImpl<TU: TodoUseCase> {
    pub tu: TU,
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

    async fn find_todo_by_id(
        &self,
        request: tonic::Request<FindTodoByIdRequest>,
    ) -> Result<tonic::Response<FindTodoByIdResponse>, tonic::Status> {
        let target_id = request.into_inner().id;
        let todo = self.tu.find_by_id(target_id).await;
        match todo {
            Ok(todo) => match todo {
                Some(todo) => {
                    let todo = Todo {
                        id: todo.id,
                        title: todo.title.unwrap_or("".to_string()),
                    };
                    let response = FindTodoByIdResponse { todo: Some(todo) };
                    return Ok(tonic::Response::new(response));
                }
                None => {
                    let response = FindTodoByIdResponse { todo: None };
                    return Ok(tonic::Response::new(response));
                }
            },
            Err(_) => {
                return Err(tonic::Status::internal("Internal Server Error".to_string()));
            }
        }
    }

    async fn create_todo(
        &self,
        request: tonic::Request<CreateTodoRequest>,
    ) -> Result<tonic::Response<CreateTodoResponse>, tonic::Status> {
        let title = request.into_inner().title;
        let todo = self.tu.create(CreateTodoDto { title }).await;
        match todo {
            Ok(todo) => {
                let todo = Todo {
                    id: todo.id,
                    title: todo.title.unwrap_or("".to_string()),
                };
                let response = CreateTodoResponse { todo: Some(todo) };
                return Ok(tonic::Response::new(response));
            }
            Err(_) => {
                return Err(tonic::Status::internal("Internal Server Error".to_string()));
            }
        }
    }

    async fn update_todo(
        &self,
        request: tonic::Request<UpdateTodoRequest>,
    ) -> Result<tonic::Response<UpdateTodoResponse>, tonic::Status> {
        let id = request.get_ref().id;
        let title = request.get_ref().title.to_string();
        let todo = self
            .tu
            .update(TodoDto {
                id,
                title: Some(title),
            })
            .await;
        match todo {
            Ok(todo) => {
                let todo = Todo {
                    id: todo.id,
                    title: todo.title.unwrap_or("".to_string()),
                };
                let response = UpdateTodoResponse { todo: Some(todo) };
                return Ok(tonic::Response::new(response));
            }
            Err(_) => {
                return Err(tonic::Status::internal("Internal Server Error".to_string()));
            }
        }
    }

    async fn delete_todo(
        &self,
        request: tonic::Request<DeleteTodoRequest>,
    ) -> Result<tonic::Response<DeleteTodoResponse>, tonic::Status> {
        let id = request.get_ref().id;
        let find_todo_result = self.tu.find_by_id(id).await;

        let todo = match find_todo_result {
            Ok(todo) => todo,
            Err(_) => None,
        };

        if todo.is_none() {
            return Err(tonic::Status::internal("Not Found".to_string()));
        }
        let todo = todo.unwrap();

        let todo_id = self.tu.delete(id).await;
        match todo_id {
            Ok(_) => {
                let todo = Todo {
                    id: todo.id,
                    title: todo.title.unwrap_or("".to_string()),
                };
                let response = DeleteTodoResponse { todo: Some(todo) };
                return Ok(tonic::Response::new(response));
            }
            Err(_) => {
                return Err(tonic::Status::internal("Internal Server Error".to_string()));
            }
        }
    }
}
