use client::rest::{create_todo, delete_todo, find_todo, get_todos, update_todo};

#[tokio::main]
async fn main() {
    // parse args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: rest_client <command>");
        return;
    }

    // command: get_todos, find_todo, create_todo, update_todo, delete_todo
    let command = &args[1];
    match command.as_str() {
        "help" => {
            println!("Usage: rest_client <command>");
            println!("Commands:");
            println!("  get_todos");
            println!("  find_todo <id>");
            println!("  create_todo <title>");
            println!("  update_todo <id> <title>");
            println!("  delete_todo <id>");
        }
        "get_todos" => {
            get_todos().await.unwrap();
        }
        "find_todo" => {
            if args.len() < 3 {
                println!("Usage: rest_client find_todo <id>");
                return;
            }
            let id = args[2].parse::<i64>().unwrap();
            find_todo(id).await.unwrap();
        }
        "create_todo" => {
            if args.len() < 3 {
                println!("Usage: rest_client create_todo <title>");
                return;
            }
            let title = args[2].clone();
            create_todo(title).await.unwrap();
        }
        "update_todo" => {
            if args.len() < 4 {
                println!("Usage: rest_client update_todo <id> <title>");
                return;
            }
            let id = args[2].parse::<i64>().unwrap();
            let title = args[3].clone();
            update_todo(id, title).await.unwrap();
        }
        "delete_todo" => {
            if args.len() < 3 {
                println!("Usage: rest_client delete_todo <id>");
                return;
            }
            let id = args[2].parse::<i64>().unwrap();
            delete_todo(id).await.unwrap();
        }
        _ => {
            println!("Usage: rest_client <command>");
        }
    }
}
