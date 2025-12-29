use chrono::{DateTime, Utc};
use diesel::prelude::*;
use crate::data::models::{NewTodo, Todo};
use crate::data::establish_connection;



#[tauri::command]
pub fn get_list() -> Result<Vec<Todo>, String> {
    use crate::data::schema::todo::dsl::*;

    let connection = &mut establish_connection();
    todo.select(Todo::as_select()).load(connection).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn add_todo(text: &str, date: &str) -> Result<Todo, String> {
    use crate::data::schema::todo;

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
pub fn finish_todo(delete_id: i32) -> Result<usize, String> {
    use crate::data::schema::todo::dsl::*;

    let connection = &mut establish_connection();

    diesel::delete(todo)
            .filter(id.eq(delete_id))
            .execute(connection)
            .map_err(|err| err.to_string())
}