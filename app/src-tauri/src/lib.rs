use argon2::{
    password_hash::{
        phc::PasswordHash,
        PasswordHasher,
        PasswordVerifier,
    },
    Argon2,
};

use chacha20poly1305::{
    aead::{Aead, AeadCore, Generate, KeyInit},
    Key, XChaCha20Poly1305, XNonce,
};

use rand::RngCore;
use rusqlite::{params, Connection, OptionalExtension};
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Default)]
struct VaultState {
    encryption_key: Mutex<Option<[u8; 32]>>,
}

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
                master_password_hash TEXT NOT NULL,
                encryption_salt BLOB
            )
            ",
            [],
        )
        .map_err(|e| format!("Could not create vault settings table: {}", e))?;

    let mut has_encryption_salt = false;

    let mut statement = connection
        .prepare("PRAGMA table_info(vault_settings)")
        .map_err(|e| format!("Could not inspect vault settings: {}", e))?;

    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|e| format!("Could not inspect vault settings: {}", e))?;

    for column in columns {
        if column
            .map_err(|e| format!("Could not inspect database column: {}", e))?
            == "encryption_salt"
        {
            has_encryption_salt = true;
            break;
        }
    }

    drop(statement);

    if !has_encryption_salt {
        connection
            .execute(
                "ALTER TABLE vault_settings ADD COLUMN encryption_salt BLOB",
                [],
            )
            .map_err(|e| format!("Could not update vault settings table: {}", e))?;
    }

    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS credentials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                encrypted_data BLOB NOT NULL,
                nonce BLOB NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            ",
            [],
        )
        .map_err(|e| format!("Could not create credentials table: {}", e))?;

    Ok(())
}

fn derive_encryption_key(
    master_password: &str,
    salt: &[u8],
) -> Result<[u8; 32], String> {
    let mut output = [0u8; 32];

    Argon2::default()
        .hash_password_into(
            master_password.as_bytes(),
            salt,
            &mut output,
        )
        .map_err(|e| format!("Could not derive encryption key: {}", e))?;

    Ok(output)
}

fn ensure_encryption_salt(
    connection: &Connection,
) -> Result<Vec<u8>, String> {
    let existing_salt: Option<Vec<u8>> = connection
        .query_row(
            "
            SELECT encryption_salt
            FROM vault_settings
            WHERE id = 1
            ",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| format!("Could not read encryption salt: {}", e))?
        .flatten();

    if let Some(salt) = existing_salt {
        return Ok(salt);
    }

    let mut salt = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);

    connection
        .execute(
            "
            UPDATE vault_settings
            SET encryption_salt = ?1
            WHERE id = 1
            ",
            params![salt.to_vec()],
        )
        .map_err(|e| format!("Could not store encryption salt: {}", e))?;

    Ok(salt.to_vec())
}

fn encrypt_data(
    plaintext: &str,
    encryption_key: &[u8; 32],
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let key = Key::from_slice(encryption_key);
    let cipher = XChaCha20Poly1305::new(key);

    let nonce = XNonce::generate();

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| "Could not encrypt credential data.".to_string())?;

    Ok((ciphertext, nonce.to_vec()))
}

fn decrypt_data(
    ciphertext: &[u8],
    nonce: &[u8],
    encryption_key: &[u8; 32],
) -> Result<String, String> {
    if nonce.len() != 24 {
        return Err("Stored credential nonce is invalid.".to_string());
    }

    let key = Key::from_slice(encryption_key);
    let cipher = XChaCha20Poly1305::new(key);

    let nonce = XNonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Could not decrypt credential data.".to_string())?;

    String::from_utf8(plaintext)
        .map_err(|_| "Decrypted credential data is invalid.".to_string())
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

    let mut encryption_salt = [0u8; 16];
    rand::rng().fill_bytes(&mut encryption_salt);

    connection
        .execute(
            "
            INSERT INTO vault_settings (
                id,
                master_password_hash,
                encryption_salt
            )
            VALUES (?1, ?2, ?3)
            ",
            params![
                1,
                password_hash,
                encryption_salt.to_vec()
            ],
        )
        .map_err(|e| format!("Could not initialise vault: {}", e))?;

    Ok("VaultFive vault created successfully.".to_string())
}

#[tauri::command]
fn verify_master_password(
    app: tauri::AppHandle,
    state: State<'_, VaultState>,
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

    if valid {
        let salt = ensure_encryption_salt(&connection)?;

        let encryption_key =
            derive_encryption_key(&master_password, &salt)?;

        let mut stored_key = state
            .encryption_key
            .lock()
            .map_err(|_| "Could not access vault state.".to_string())?;

        *stored_key = Some(encryption_key);
    }

    Ok(valid)
}

#[tauri::command]
fn lock_vault(
    state: State<'_, VaultState>,
) -> Result<(), String> {
    let mut stored_key = state
        .encryption_key
        .lock()
        .map_err(|_| "Could not access vault state.".to_string())?;

    *stored_key = None;

    Ok(())
}

#[tauri::command]
fn test_secure_storage(
    app: tauri::AppHandle,
    state: State<'_, VaultState>,
) -> Result<String, String> {
    let encryption_key = {
        let stored_key = state
            .encryption_key
            .lock()
            .map_err(|_| "Could not access vault state.".to_string())?;

        stored_key
            .as_ref()
            .copied()
            .ok_or_else(|| "Vault is locked.".to_string())?
    };

    let connection = get_database_connection(&app)?;

    initialise_database(&connection)?;

    let test_data = r#"{"service":"VaultFive Test","username":"test-user","password":"Secure-Test-Password"}"#;

    let (ciphertext, nonce) =
        encrypt_data(test_data, &encryption_key)?;

    connection
        .execute(
            "
            INSERT INTO credentials (
                encrypted_data,
                nonce
            )
            VALUES (?1, ?2)
            ",
            params![
                ciphertext,
                nonce
            ],
        )
        .map_err(|e| format!("Could not write encrypted test record: {}", e))?;

    let record_id = connection.last_insert_rowid();

    let (stored_ciphertext, stored_nonce): (Vec<u8>, Vec<u8>) = connection
        .query_row(
            "
            SELECT encrypted_data, nonce
            FROM credentials
            WHERE id = ?1
            ",
            params![record_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Could not read encrypted test record: {}", e))?;

    let decrypted =
        decrypt_data(
            &stored_ciphertext,
            &stored_nonce,
            &encryption_key,
        )?;

    connection
        .execute(
            "DELETE FROM credentials WHERE id = ?1",
            params![record_id],
        )
        .map_err(|e| format!("Could not remove test record: {}", e))?;

    if decrypted != test_data {
        return Err(
            "Secure storage verification failed.".to_string()
        );
    }

    Ok(
        "Secure storage verified: data encrypted, persisted, read and decrypted successfully."
            .to_string()
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(VaultState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            test_database,
            check_vault_exists,
            initialise_vault,
            verify_master_password,
            lock_vault,
            test_secure_storage
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
