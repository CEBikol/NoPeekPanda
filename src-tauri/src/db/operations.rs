use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

/// Краткое описание сервиса (без пароля)
#[derive(Serialize)]
pub struct ServiceSummary {
    pub id: u64,
    pub site: String,
    pub login: String,
}

/// Полная запись (включая пароль)
#[derive(Serialize)]
pub struct PasswordEntry {
    pub id: u64,
    pub site: String,
    pub login: String,
    pub password: String,
}

/// Возвращает список всех сервисов (без паролей)
pub fn list_services(conn: &Connection) -> Result<Vec<ServiceSummary>> {
    let mut stmt = conn
        .prepare("SELECT id, site, login FROM passwords")
        .context("Failed to prepare service list query")?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ServiceSummary {
                id: row.get(0)?,
                site: row.get(1)?,
                login: row.get(2)?,
            })
        })
        .context("Failed to execute service list query")?;

    let mut services = Vec::new();
    for row in rows {
        services.push(row.context("Failed to parse service entry")?);
    }

    Ok(services)
}

/// Возвращает пароль по ID
pub fn get_password(conn: &Connection, id: u64) -> Result<PasswordEntry> {
    conn.query_row(
        "SELECT id, site, login, password FROM passwords WHERE id = ?1",
        params![id],
        |row| {
            Ok(PasswordEntry {
                id: row.get(0)?,
                site: row.get(1)?,
                login: row.get(2)?,
                password: row.get(3)?,
            })
        },
    )
    .context("Failed to fetch password entry")
}

/// Добавляет новую запись
pub fn add_password(conn: &Connection, site: &str, login: &str, password: &str) -> Result<u64> {
    conn.execute(
        "INSERT INTO passwords (site, login, password) VALUES (?1, ?2, ?3)",
        params![site, login, password],
    )
    .context("Failed to insert new password")?;

    Ok(conn.last_insert_rowid() as u64)
}

/// Удаляет запись по ID
pub fn delete_password(conn: &Connection, id: u64) -> Result<()> {
    conn.execute("DELETE FROM passwords WHERE id = ?1", params![id])
        .context("Failed to delete password entry")?;

    Ok(())
}
