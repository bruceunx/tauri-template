#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod db;

use sqlx::{MySql, Pool};
use tokio::runtime::Runtime;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let pool: Pool<MySql> = Runtime::new().unwrap().block_on(db::connect());
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db::MySqlPoolWrapper { pool })
        .invoke_handler(tauri::generate_handler![
            greet,
            client::greet_client,
            client::stream_client,
            db::insert_data,
            db::get_all_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
