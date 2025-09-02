mod connection;
pub mod operations;

use anyhow::{Context, Result};
use rusqlite::Connection;
use secrecy::SecretBox as Secret;
use std::ops::Not;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Основная структура для работы с зашифрованным хранилищем
pub struct Vault {
    inner: Arc<Mutex<VaultInner>>,
}

struct VaultInner {
    path: PathBuf,
    connection: Option<Connection>,
    is_locked: bool,
}

impl Vault {
    /// Создаёт новый экземпляр менеджера хранилища
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            inner: Arc::new(Mutex::new(VaultInner {
                path: path.into(),
                connection: None,
                is_locked: true,
            })),
        }
    }

    /// Возвращает копию пути к файлу хранилища
    pub fn path(&self) -> PathBuf {
        self.inner
            .lock()
            .map(|inner| inner.path.clone())
            .unwrap_or_else(|_| {
                self.inner
                    .lock()
                    .expect("Critical: poisoned mutex")
                    .path
                    .clone()
            })
    }

    /// Возвращает внутреннее состояние (для внутреннего использования)
    pub fn inner(&self) -> std::sync::MutexGuard<'_, VaultInner> {
        self.inner.lock().expect("Failed to acquire database lock")
    }

    /// Разблокирует хранилище с помощью мастер-пароля
    pub fn unlock(&self, master_password: Secret<String>) -> Result<()> {
        let mut inner = self.inner();

        if !inner.is_locked {
            return Ok(());
        }

        let conn = connection::open_existing_storage(&inner.path, master_password)
            .context("Failed to open storage")?;

        inner.connection = Some(conn);
        inner.is_locked = false;

        Ok(())
    }

    /// Блокирует хранилище
    pub fn lock(&self) -> Result<()> {
        let mut inner = self.inner();

        inner.connection = None;
        inner.is_locked = true;

        Ok(())
    }

    /// Проверяет, разблокировано ли хранилище
    pub fn is_unlocked(&self) -> bool {
        self.inner().is_locked.not()
    }

    /// Возвращает список всех сервисов
    pub fn list_services(&self) -> Result<Vec<operations::ServiceSummary>> {
        let inner = self.inner();

        let conn = inner
            .connection
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vault is locked"))?;

        operations::list_services(conn)
    }

    /// Возвращает пароль по ID
    pub fn get_password(&self, id: u64) -> Result<operations::PasswordEntry> {
        let inner = self.inner();

        let conn = inner
            .connection
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vault is locked"))?;

        operations::get_password(conn, id)
    }

    /// Добавляет новую запись
    pub fn add_password(&self, site: &str, login: &str, password: &str) -> Result<u64> {
        let inner = self.inner();

        let conn = inner
            .connection
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vault is locked"))?;

        operations::add_password(conn, site, login, password)
    }

    /// Удаляет запись по ID
    pub fn delete_password(&self, id: u64) -> Result<()> {
        let inner = self.inner();

        let conn = inner
            .connection
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vault is locked"))?;

        operations::delete_password(conn, id)
    }
}

impl Drop for Vault {
    fn drop(&mut self) {
        let _ = self.lock();
    }
}

/// Создаёт новое хранилище
pub fn create_new_vault<P: Into<PathBuf>>(
    path: P,
    master_password: Secret<String>,
) -> Result<Vault> {
    let path = path.into();

    if path.exists() {
        anyhow::bail!("Storage file already exists");
    }

    connection::create_new_storage(&path, master_password).context("Failed to create storage")?;

    Ok(Vault::new(path))
}

/// Проверяет существование хранилища
pub fn vault_exists<P: AsRef<std::path::Path>>(path: P) -> bool {
    path.as_ref().is_file()
}
