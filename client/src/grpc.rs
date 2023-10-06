use presentation::grpc::proto_impl::{
    CreateTodoRequest, DeleteTodoRequest, FindTodoByIdRequest, GetTodosRequest, TodoServiceClient,
    UpdateTodoRequest,
};
use tonic::Request;

pub async fn get_todos() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoServiceClient::connect("http://localhost:8081").await?;

    let request = Request::new(GetTodosRequest {});

    let response = client.get_todos(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

pub async fn find_todo(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoServiceClient::connect("http://localhost:8081").await?;

    let request = Request::new(FindTodoByIdRequest { id });

    let response = client.find_todo_by_id(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

pub async fn create_todo(title: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoServiceClient::connect("http://localhost:8081").await?;

    let request = Request::new(CreateTodoRequest { title });

    let response = client.create_todo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

pub async fn update_todo(id: i64, title: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoServiceClient::connect("http://localhost:8081").await?;

    let request = Request::new(UpdateTodoRequest { id, title });

    let response = client.update_todo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

pub async fn delete_todo(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoServiceClient::connect("http://localhost:8081").await?;

    let request = Request::new(DeleteTodoRequest { id });

    let response = client.delete_todo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
