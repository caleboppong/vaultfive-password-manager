use argon2::{
    password_hash::{
        phc::PasswordHash,
        PasswordHasher,
        PasswordVerifier,
    },
    Argon2,
};

use chacha20poly1305::{
    aead::{Aead, Generate, KeyInit},
    XChaCha20Poly1305, XNonce,
};

use rand::RngCore;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Default)]
struct VaultState {
    encryption_key: Mutex<Option<[u8; 32]>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CredentialData {
    service: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct Credential {
    id: i64,
    service: String,
    username: String,
    password: String,
    created_at: String,
    updated_at: String,
}

fn get_database_connection(
    app: &tauri::AppHandle,
) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| {
            format!("Could not find app data directory: {}", e)
        })?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| {
            format!("Could not create app data directory: {}", e)
        })?;

    let database_path = app_data_dir.join("vaultfive.db");

    Connection::open(database_path)
        .map_err(|e| {
            format!("Could not open database: {}", e)
        })
}

fn initialise_database(
    connection: &Connection,
) -> Result<(), String> {
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
        .map_err(|e| {
            format!("Could not create vault settings table: {}", e)
        })?;

    let mut has_encryption_salt = false;

    let mut statement = connection
        .prepare("PRAGMA table_info(vault_settings)")
        .map_err(|e| {
            format!("Could not inspect vault settings: {}", e)
        })?;

    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|e| {
            format!("Could not inspect vault settings: {}", e)
        })?;

    for column in columns {
        if column
            .map_err(|e| {
                format!("Could not inspect database column: {}", e)
            })?
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
                "
                ALTER TABLE vault_settings
                ADD COLUMN encryption_salt BLOB
                ",
                [],
            )
            .map_err(|e| {
                format!("Could not update vault settings table: {}", e)
            })?;
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
        .map_err(|e| {
            format!("Could not create credentials table: {}", e)
        })?;

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
        .map_err(|e| {
            format!("Could not derive encryption key: {}", e)
        })?;

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
        .map_err(|e| {
            format!("Could not read encryption salt: {}", e)
        })?
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
        .map_err(|e| {
            format!("Could not store encryption salt: {}", e)
        })?;

    Ok(salt.to_vec())
}

fn session_key_from_vault_state(state: &VaultState) -> Result<[u8; 32], String> {
    let key_guard = state
        .encryption_key
        .lock()
        .map_err(|_| "Could not access vault state.".to_string())?;

    key_guard.ok_or_else(|| "Vault is locked.".to_string())
}

fn get_session_key(state: &State<'_, VaultState>) -> Result<[u8; 32], String> {
    session_key_from_vault_state(state.inner())
}

fn encrypt_data(
    plaintext: &str,
    encryption_key: &[u8; 32],
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cipher = XChaCha20Poly1305::new_from_slice(
        encryption_key,
    )
    .map_err(|_| {
        "Could not create encryption cipher.".to_string()
    })?;

    let nonce = XNonce::generate();

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| {
            "Could not encrypt credential data.".to_string()
        })?;

    Ok((ciphertext, nonce.to_vec()))
}

fn decrypt_data(
    ciphertext: &[u8],
    nonce: &[u8],
    encryption_key: &[u8; 32],
) -> Result<String, String> {
    let cipher = XChaCha20Poly1305::new_from_slice(
        encryption_key,
    )
    .map_err(|_| {
        "Could not create encryption cipher.".to_string()
    })?;

    let nonce = XNonce::try_from(nonce)
        .map_err(|_| {
            "Stored credential nonce is invalid.".to_string()
        })?;

    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| {
            "Could not decrypt credential data.".to_string()
        })?;

    String::from_utf8(plaintext)
        .map_err(|_| {
            "Decrypted credential data is invalid.".to_string()
        })
}

fn validate_credential(
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), String> {
    if service.trim().is_empty() {
        return Err(
            "Service name is required.".to_string()
        );
    }

    if username.trim().is_empty() {
        return Err(
            "Username or email is required.".to_string()
        );
    }

    if password.is_empty() {
        return Err(
            "Password is required.".to_string()
        );
    }

    if service.trim().len() > 100 {
        return Err(
            "Service name must be 100 characters or fewer."
                .to_string()
        );
    }

    if username.trim().len() > 200 {
        return Err(
            "Username must be 200 characters or fewer."
                .to_string()
        );
    }

    if password.len() > 500 {
        return Err(
            "Password must be 500 characters or fewer."
                .to_string()
        );
    }

    Ok(())
}

#[tauri::command]
fn check_vault_exists(
    app: tauri::AppHandle,
) -> Result<bool, String> {
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
        .map_err(|e| {
            format!("Could not check vault status: {}", e)
        })?;

    Ok(count > 0)
}

#[tauri::command]
fn initialise_vault(
    app: tauri::AppHandle,
    master_password: String,
) -> Result<String, String> {
    if master_password.trim().is_empty() {
        return Err(
            "Master password is required.".to_string()
        );
    }

    if master_password.len() < 8 {
        return Err(
            "Master password must contain at least 8 characters."
                .to_string()
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
        .map_err(|e| {
            format!("Could not check existing vault: {}", e)
        })?;

    if existing_count > 0 {
        return Err(
            "A VaultFive vault has already been initialised."
                .to_string()
        );
    }

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(
            master_password.as_bytes(),
        )
        .map_err(|e| {
            format!(
                "Could not protect master password: {}",
                e
            )
        })?
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
        .map_err(|e| {
            format!("Could not initialise vault: {}", e)
        })?;

    Ok(
        "VaultFive vault created successfully."
            .to_string()
    )
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
        .map_err(|_| {
            "Vault has not been initialised."
                .to_string()
        })?;

    let parsed_hash = PasswordHash::new(
        &stored_hash,
    )
    .map_err(|e| {
        format!(
            "Stored password hash is invalid: {}",
            e
        )
    })?;

    let argon2 = Argon2::default();

    let valid = argon2
        .verify_password(
            master_password.as_bytes(),
            &parsed_hash,
        )
        .is_ok();

    if valid {
        let salt =
            ensure_encryption_salt(&connection)?;

        let encryption_key =
            derive_encryption_key(
                &master_password,
                &salt,
            )?;

        let mut stored_key = state
            .encryption_key
            .lock()
            .map_err(|_| {
                "Could not access vault state."
                    .to_string()
            })?;

        *stored_key = Some(encryption_key);
    } else {
        let mut stored_key = state
            .encryption_key
            .lock()
            .map_err(|_| {
                "Could not access vault state."
                    .to_string()
            })?;

        *stored_key = None;
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
        .map_err(|_| {
            "Could not access vault state."
                .to_string()
        })?;

    *stored_key = None;

    Ok(())
}

#[tauri::command]
fn add_credential(
    app: tauri::AppHandle,
    state: State<'_, VaultState>,
    service: String,
    username: String,
    password: String,
) -> Result<i64, String> {
    validate_credential(
        &service,
        &username,
        &password,
    )?;

    let encryption_key =
        get_session_key(&state)?;

    let credential = CredentialData {
        service: service.trim().to_string(),
        username: username.trim().to_string(),
        password,
    };

    let plaintext =
        serde_json::to_string(&credential)
            .map_err(|e| {
                format!(
                    "Could not prepare credential: {}",
                    e
                )
            })?;

    let (ciphertext, nonce) =
        encrypt_data(
            &plaintext,
            &encryption_key,
        )?;

    let connection =
        get_database_connection(&app)?;

    initialise_database(&connection)?;

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
        .map_err(|e| {
            format!(
                "Could not save credential: {}",
                e
            )
        })?;

    Ok(connection.last_insert_rowid())
}

#[tauri::command]
fn get_credentials(
    app: tauri::AppHandle,
    state: State<'_, VaultState>,
) -> Result<Vec<Credential>, String> {
    let encryption_key =
        get_session_key(&state)?;

    let connection =
        get_database_connection(&app)?;

    initialise_database(&connection)?;

    let mut statement = connection
        .prepare(
            "
            SELECT
                id,
                encrypted_data,
                nonce,
                created_at,
                updated_at
            FROM credentials
            ORDER BY id DESC
            ",
        )
        .map_err(|e| {
            format!(
                "Could not prepare credential query: {}",
                e
            )
        })?;

    let rows = statement
        .query_map(
            [],
            |row| {
                let id: i64 =
                    row.get(0)?;

                let encrypted_data: Vec<u8> =
                    row.get(1)?;

                let nonce: Vec<u8> =
                    row.get(2)?;

                let created_at: String =
                    row.get(3)?;

                let updated_at: String =
                    row.get(4)?;

                Ok((
                    id,
                    encrypted_data,
                    nonce,
                    created_at,
                    updated_at,
                ))
            },
        )
        .map_err(|e| {
            format!(
                "Could not read credentials: {}",
                e
            )
        })?;

    let mut credentials =
        Vec::new();

    for row in rows {
        let (
            id,
            encrypted_data,
            nonce,
            created_at,
            updated_at,
        ) = row.map_err(|e| {
            format!(
                "Could not read credential row: {}",
                e
            )
        })?;

        let plaintext = decrypt_data(
            &encrypted_data,
            &nonce,
            &encryption_key,
        )?;

        let credential_data: CredentialData =
            serde_json::from_str(
                &plaintext,
            )
            .map_err(|e| {
                format!(
                    "Could not decode credential: {}",
                    e
                )
            })?;

        credentials.push(
            Credential {
                id,
                service:
                    credential_data.service,
                username:
                    credential_data.username,
                password:
                    credential_data.password,
                created_at,
                updated_at,
            },
        );
    }

    Ok(credentials)
}

#[tauri::command]
fn update_credential(
    app: tauri::AppHandle,
    state: State<'_, VaultState>,
    id: i64,
    service: String,
    username: String,
    password: String,
) -> Result<(), String> {
    validate_credential(
        &service,
        &username,
        &password,
    )?;

    let encryption_key =
        get_session_key(&state)?;

    let credential = CredentialData {
        service: service.trim().to_string(),
        username: username.trim().to_string(),
        password,
    };

    let plaintext =
        serde_json::to_string(
            &credential,
        )
        .map_err(|e| {
            format!(
                "Could not prepare credential: {}",
                e
            )
        })?;

    let (ciphertext, nonce) =
        encrypt_data(
            &plaintext,
            &encryption_key,
        )?;

    let connection =
        get_database_connection(&app)?;

    initialise_database(&connection)?;

    let rows_updated = connection
        .execute(
            "
            UPDATE credentials
            SET
                encrypted_data = ?1,
                nonce = ?2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?3
            ",
            params![
                ciphertext,
                nonce,
                id
            ],
        )
        .map_err(|e| {
            format!(
                "Could not update credential: {}",
                e
            )
        })?;

    if rows_updated == 0 {
        return Err(
            "Credential was not found."
                .to_string()
        );
    }

    Ok(())
}

#[tauri::command]
fn delete_credential(
    app: tauri::AppHandle,
    state: State<'_, VaultState>,
    id: i64,
) -> Result<(), String> {
    get_session_key(&state)?;

    let connection =
        get_database_connection(&app)?;

    initialise_database(&connection)?;

    let rows_deleted = connection
        .execute(
            "
            DELETE FROM credentials
            WHERE id = ?1
            ",
            params![id],
        )
        .map_err(|e| {
            format!(
                "Could not delete credential: {}",
                e
            )
        })?;

    if rows_deleted == 0 {
        return Err(
            "Credential was not found."
                .to_string()
        );
    }

    Ok(())
}

#[cfg_attr(
    mobile,
    tauri::mobile_entry_point
)]
pub fn run() {
    tauri::Builder::default()
        .manage(
            VaultState::default(),
        )
        .plugin(
            tauri_plugin_opener::init(),
        )
        .invoke_handler(
            tauri::generate_handler![
                check_vault_exists,
                initialise_vault,
                verify_master_password,
                lock_vault,
                add_credential,
                get_credentials,
                update_credential,
                delete_credential
            ],
        )
        .run(
            tauri::generate_context!(),
        )
        .expect(
            "error while running Tauri application",
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locked_vault_rejects_session_key_access() {
        let state = VaultState {
            encryption_key: Mutex::new(None),
        };

        let result = session_key_from_vault_state(&state);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Vault is locked.");
    }

    #[test]
    fn unlocked_vault_allows_session_key_access() {
        let expected_key = [42u8; 32];

        let state = VaultState {
            encryption_key: Mutex::new(Some(expected_key)),
        };

        let result = session_key_from_vault_state(&state);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_key);
    }
}