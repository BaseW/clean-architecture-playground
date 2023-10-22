// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use client::rest::get_todos as get_todos_by_rest;
use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
}

#[tauri::command]
async fn get_todos() -> Result<Vec<Todo>, String> {
    let todos_response = get_todos_by_rest().await;
    match todos_response {
        Ok(res) => match res.todos {
            Some(todos) => {
                let mut todos_vec = Vec::new();
                for todo in todos {
                    todos_vec.push(Todo {
                        id: todo.id,
                        title: todo.title.unwrap_or("".to_string()),
                    });
                }
                Ok(todos_vec)
            }
            None => {
                println!("No todos found");
                Ok(Vec::new())
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Err(err.to_string())
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
