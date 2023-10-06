use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct CreateTodo;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct UpdateTodo;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct DeleteTodo;

pub async fn create_todo(title: String) -> Result<(), Box<dyn std::error::Error>> {
    let request_body = CreateTodo::build_query(create_todo::Variables { title });
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/graphql")
        .json(&request_body)
        .send()
        .await?;
    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}

pub async fn update_todo(id: i64, title: String) -> Result<(), Box<dyn std::error::Error>> {
    let request_body = UpdateTodo::build_query(update_todo::Variables { id, title });
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/graphql")
        .json(&request_body)
        .send()
        .await?;
    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}

pub async fn delete_todo(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let request_body = DeleteTodo::build_query(delete_todo::Variables { id });
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/graphql")
        .json(&request_body)
        .send()
        .await?;
    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}
