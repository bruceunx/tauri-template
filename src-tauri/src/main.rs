// Prevents additional console window on Windows in release, DO NOT REMOVE!!j
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, client::greet_client,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
