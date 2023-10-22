use presentation::rest::object::TodosResponse;
use reqwest::Client;

pub async fn get_todos() -> Result<TodosResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("http://localhost:8080/todos").send().await?;
    println!("Status: {}", res.status());
    let body = res.json::<TodosResponse>().await;
    match body {
        Ok(body) => {
            println!("Body: {:?}", body);
            Ok(body)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Err(Box::new(err))
        }
    }
}

pub async fn find_todo(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .get(format!("http://localhost:8080/todos/{}", id))
        .send()
        .await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);
    Ok(())
}

pub async fn create_todo(title: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("http://localhost:8080/todos")
        .json(&serde_json::json!({
            "title": title,
        }))
        .send()
        .await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);
    Ok(())
}

pub async fn update_todo(id: i64, title: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .put("http://localhost:8080/todos")
        .json(&serde_json::json!({
            "id": id,
            "title": title,
        }))
        .send()
        .await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);
    Ok(())
}

pub async fn delete_todo(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .delete("http://localhost:8080/todos")
        .json(&serde_json::json!({
            "id": id,
        }))
        .send()
        .await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);
    Ok(())
}
