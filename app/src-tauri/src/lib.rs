use rusqlite::{params, Connection};
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test_database(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| e.to_string())?;

    let db_path = app_data_dir.join("vaultfive.db");

    let conn = Connection::open(&db_path)
        .map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS poc_test (
            id INTEGER PRIMARY KEY,
            message TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO poc_test (id, message)
         VALUES (?1, ?2)",
        params![1, "VaultFive SQLite connection successful"],
    )
    .map_err(|e| e.to_string())?;

    let message: String = conn
        .query_row(
            "SELECT message FROM poc_test WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(format!("SQLite connected successfully: {}", message))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
