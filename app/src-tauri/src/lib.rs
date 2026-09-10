use argon2::{
    password_hash::{
        phc::PasswordHash,
        PasswordHasher,
        PasswordVerifier,
    },
    Argon2,
};

use rusqlite::{params, Connection};
use tauri::Manager;

fn get_database_connection(app: &tauri::AppHandle) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not find app data directory: {}", e))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Could not create app data directory: {}", e))?;

    let database_path = app_data_dir.join("vaultfive.db");

    Connection::open(database_path)
        .map_err(|e| format!("Could not open database: {}", e))
}

fn initialise_database(connection: &Connection) -> Result<(), String> {
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS vault_settings (
                id INTEGER PRIMARY KEY,
                master_password_hash TEXT NOT NULL
            )
            ",
            [],
        )
        .map_err(|e| format!("Could not create vault settings table: {}", e))?;

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test_database(app: tauri::AppHandle) -> Result<String, String> {
    let connection = get_database_connection(&app)?;

    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS poc_test (
                id INTEGER PRIMARY KEY,
                message TEXT NOT NULL
            )
            ",
            [],
        )
        .map_err(|e| format!("Could not create PoC table: {}", e))?;

    connection
        .execute(
            "
            INSERT OR REPLACE INTO poc_test (
                id,
                message
            )
            VALUES (?1, ?2)
            ",
            params![1, "VaultFive SQLite connection successful"],
        )
        .map_err(|e| format!("Could not insert PoC data: {}", e))?;

    let message: String = connection
        .query_row(
            "
            SELECT message
            FROM poc_test
            WHERE id = 1
            ",
            [],
            |row| row.get(0),
        )
        .map_err(|e| format!("Could not read PoC data: {}", e))?;

    Ok(format!("SQLite connected successfully: {}", message))
}

#[tauri::command]
fn check_vault_exists(app: tauri::AppHandle) -> Result<bool, String> {
    let connection = get_database_connection(&app)?;

    initialise_database(&connection)?;

    let count: i64 = connection
        .query_row(
            "
            SELECT COUNT(*)
            FROM vault_settings
            WHERE id = 1
            ",
            [],
            |row| row.get(0),
        )
        .map_err(|e| format!("Could not check vault status: {}", e))?;

    Ok(count > 0)
}

#[tauri::command]
fn initialise_vault(
    app: tauri::AppHandle,
    master_password: String,
) -> Result<String, String> {
    if master_password.trim().is_empty() {
        return Err("Master password is required.".to_string());
    }

    if master_password.len() < 8 {
        return Err(
            "Master password must contain at least 8 characters.".to_string()
        );
    }

    let connection = get_database_connection(&app)?;

    initialise_database(&connection)?;

    let existing_count: i64 = connection
        .query_row(
            "
            SELECT COUNT(*)
            FROM vault_settings
            WHERE id = 1
            ",
            [],
            |row| row.get(0),
        )
        .map_err(|e| format!("Could not check existing vault: {}", e))?;

    if existing_count > 0 {
        return Err(
            "A VaultFive vault has already been initialised.".to_string()
        );
    }

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(master_password.as_bytes())
        .map_err(|e| format!("Could not protect master password: {}", e))?
        .to_string();

    connection
        .execute(
            "
            INSERT INTO vault_settings (
                id,
                master_password_hash
            )
            VALUES (?1, ?2)
            ",
            params![1, password_hash],
        )
        .map_err(|e| format!("Could not initialise vault: {}", e))?;

    Ok("VaultFive vault created successfully.".to_string())
}

#[tauri::command]
fn verify_master_password(
    app: tauri::AppHandle,
    master_password: String,
) -> Result<bool, String> {
    if master_password.is_empty() {
        return Ok(false);
    }

    let connection = get_database_connection(&app)?;

    initialise_database(&connection)?;

    let stored_hash: String = connection
        .query_row(
            "
            SELECT master_password_hash
            FROM vault_settings
            WHERE id = 1
            ",
            [],
            |row| row.get(0),
        )
        .map_err(|_| "Vault has not been initialised.".to_string())?;

    let parsed_hash = PasswordHash::new(&stored_hash)
        .map_err(|e| format!("Stored password hash is invalid: {}", e))?;

    let argon2 = Argon2::default();

    let valid = argon2
        .verify_password(
            master_password.as_bytes(),
            &parsed_hash,
        )
        .is_ok();

    Ok(valid)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            test_database,
            check_vault_exists,
            initialise_vault,
            verify_master_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
