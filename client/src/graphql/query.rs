use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct GetTodos;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct FindTodo;

pub async fn get_todos() -> Result<(), Box<dyn std::error::Error>> {
    let request_body = GetTodos::build_query(get_todos::Variables {});
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

pub async fn find_todo(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let request_body = FindTodo::build_query(find_todo::Variables { id });
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
