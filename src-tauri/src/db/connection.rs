use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use secrecy::{ExposeSecret, SecretBox as Secret};
use std::path::Path;

/// Настройки безопасности для SQLCipher
const CIPHER_SETTINGS: &[(&str, &str)] = &[
    ("cipher_page_size", "4096"),
    ("kdf_iter", "64000"),
    ("cipher_hmac_algorithm", "HMAC_SHA512"),
    ("cipher_kdf_algorithm", "PBKDF2_HMAC_SHA512"),
];

/// Проверяет, существует ли хранилище по указанному пути
pub fn storage_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Создаёт новое хранилище (только если файла ещё нет)
pub fn create_new_storage<P: AsRef<Path>>(
    path: P,
    master_password: Secret<String>,
) -> Result<Connection> {
    let path = path.as_ref();

    // Проверяем, что файла ещё нет
    if path.exists() {
        anyhow::bail!("Storage file already exists: {:?}", path);
    }

    // Создаём директорию, если нужно
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create storage directory: {:?}", parent))?;
    }

    // Создаём новое соединение с флагом CREATE
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )
    .with_context(|| format!("Failed to create storage file: {:?}", path))?;

    // Устанавливаем шифрование
    setup_encryption(&conn, &master_password)
        .context("Failed to initialize encryption for new storage")?;

    // Инициализируем структуру БД
    initialize_storage_schema(&conn).context("Failed to initialize new storage schema")?;

    // Проверяем целостность только что созданной БД
    verify_integrity(&conn).context("Critical error: newly created storage is corrupted")?;

    Ok(conn)
}

/// Открывает существующее хранилище (только если файл существует)
pub fn open_existing_storage<P: AsRef<Path>>(
    path: P,
    master_password: Secret<String>,
) -> Result<Connection> {
    let path = path.as_ref();

    // КРИТИЧЕСКАЯ ПРОВЕРКА: файл ДОЛЖЕН существовать
    if !path.exists() {
        anyhow::bail!("Storage file not found: {:?}", path);
    }
    if !path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", path);
    }

    // Открываем без флага CREATE (иначе rusqlite создаст пустой файл!)
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)
        .with_context(|| format!("Failed to open storage file: {:?}", path))?;

    // Устанавливаем шифрование
    setup_encryption(&conn, &master_password).context("Invalid password or corrupted storage")?;

    // Проверяем целостность существующей БД
    verify_integrity(&conn)
        .context("Storage corruption detected. Possible causes: wrong password or disk error")?;

    Ok(conn)
}

/// Общая логика установки шифрования
fn setup_encryption(conn: &Connection, master_password: &Secret<String>) -> Result<()> {
    // Устанавливаем ключ шифрования
    conn.pragma_update(None, "key", master_password.expose_secret().as_str())
        .context("Failed to set encryption key")?;

    // Применяем настройки шифрования
    for (pragma, value) in CIPHER_SETTINGS {
        conn.pragma_update(None, pragma, value)
            .with_context(|| format!("Failed to configure {}: {}", pragma, value))?;
    }

    Ok(())
}

/// Инициализирует структуру новой БД
fn initialize_storage_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE passwords (
            id INTEGER PRIMARY KEY,
            site TEXT NOT NULL,
            login TEXT NOT NULL,
            password TEXT NOT NULL
        );
        CREATE INDEX idx_site ON passwords(site);
        "#,
    )
    .context("Failed to initialize database schema")?;
    Ok(())
}

/// Проверяет целостность расшифрованной БД
pub fn verify_integrity(conn: &Connection) -> Result<()> {
    conn.query_row("PRAGMA integrity_check", [], |_| Ok(()))
        .context("Database corruption detected")?;
    Ok(())
}
