mod services;
mod data;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    data::establish_connection();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, 
            services::todo::get_list, 
            services::todo::add_todo, 
            services::todo::finish_todo,
            services::lane::get_price])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
