pub mod models;
pub mod schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewTodo, Todo};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_list() -> Result<Vec<Todo>, String> {
    use self::schema::todo::dsl::*;

    let connection = &mut establish_connection();
    todo.select(Todo::as_select()).load(connection).map_err(|err| err.to_string())
}

#[tauri::command]
fn add_todo(text: &str, date: &str) -> Result<Todo, String> {
    use crate::schema::todo;

    let date_time: DateTime<Utc> = date.parse().unwrap();
    
    let new_todo = NewTodo::new(text.to_string(), date_time.date_naive());
    let connection = &mut establish_connection();

    diesel::insert_into(todo::table)
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(connection)
            .map_err(|err| err.to_string())
}

#[tauri::command]
fn finish_todo(delete_id: i32) -> Result<usize, String> {
    use self::schema::todo::dsl::*;

    let connection = &mut establish_connection();

    diesel::delete(todo)
            .filter(id.eq(delete_id))
            .execute(connection)
            .map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    establish_connection();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_list, add_todo, finish_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
