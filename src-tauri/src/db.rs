#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use sqlx::Result;
use sqlx::{MySql, MySqlPool, Pool, Row};
use tauri::State;

#[allow(dead_code)]
pub struct MySqlPoolWrapper {
    pub pool: Pool<MySql>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DataEntry {
    pub(crate) name: String,
    pub(crate) age: i32,
}

// "CREATE TABLE IF NOT EXISTS data (
//     id INTEGER PRIMARY KEY,
//     name TEXT NOT NULL,
//     age INTEGER NOT NULL
// )",
//
#[tauri::command]
pub(crate) async fn connect() -> Pool<MySql> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    return MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");
}

#[tauri::command]
pub(crate) async fn insert_data(
    state: State<'_, MySqlPoolWrapper>,
    name: String,
    age: i32,
) -> Result<(), String> {
    println!("insert_data: {} {}", &name, age);
    println!("state: {:?}", &state.pool);
    let entry = DataEntry {
        name: name.to_string(),
        age,
    };
    let res = sqlx::query("INSERT INTO data (name, age) VALUES (?, ?)")
        .bind(&entry.name)
        .bind(entry.age)
        .execute(&state.pool)
        .await
        .expect("Failed to insert data");
    println!("Data inserted: {:?}", res);
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_all_data(
    state: State<'_, MySqlPoolWrapper>,
) -> Result<Vec<DataEntry>, String> {
    let result = sqlx::query("select name, age from data")
        .map(|row: sqlx::mysql::MySqlRow| DataEntry {
            name: row.get(0),
            age: row.get(1),
        })
        .fetch_all(&state.pool)
        .await
        .expect("Failed to fetch data");

    return Ok(result);
}

// #[tauri::command]
// pub(crate) async fn update_data(entry: &DataEntry, id: i32) -> Result<()> {
//     let conn = Connection::open("db.sqlite").unwrap();
//     conn.execute(
//         "UPDATE data SET name = ?1, age = ?2 WHERE id = ?3",
//         [
//             entry.name.to_string(),
//             entry.age.to_string(),
//             id.to_string(),
//         ],
//     )
//     .unwrap();
//     println!("Data updated id = {}", id);
//     Ok(())
// }
//
// #[tauri::command]
// pub(crate) fn delete_data(id: i32) -> Result<()> {
//     let conn = Connection::open("db.sqlite").unwrap();
//     conn.execute("DELETE FROM data WHERE id = ?1", [id.to_string()])
//         .unwrap();
//     println!("Data deleted id = {}", id);
//     Ok(())
// }
