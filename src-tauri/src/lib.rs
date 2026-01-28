use anyhow::anyhow;
use secrecy::SecretBox as Secret;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::State;
use tauri_plugin_updater::UpdaterExt;
use serde::Serialize;

mod db;
//mod utils;
use utils::settings;
pub mod utils;

use db::Vault;

// Состояние, которое будет храниться в Tauri
struct AppState {
    vault: Arc<Mutex<Option<Vault>>>,
}

#[derive(Serialize)]
struct UpdateInfo {
    version: String,
    body: Option<String>,
}
/// Загружает текущие настройки приложения
#[tauri::command]
async fn get_settings() -> Result<settings::AppSettings, String> {
    settings::AppSettings::load().map_err(|_| "Failed to load settings".to_string())
}

/// Обновляет настройки приложения
#[tauri::command]
async fn update_settings(theme: String, vault_folder_path: String) -> Result<(), String> {
    let mut settings =
        settings::AppSettings::load().map_err(|_| "Failed to load settings".to_string())?;

    settings.theme = theme;
    settings.vault_folder_path = vault_folder_path;

    settings
        .save()
        .map_err(|_| "Failed to save settings".to_string())?;

    Ok(())
}

/// Создаёт новое зашифрованное хранилище
#[tauri::command]
async fn create_vault(storage_name: String, password: String) -> Result<(), String> {
    let password = Secret::new(Box::new(password));

    tauri::async_runtime::spawn_blocking(move || {
        let settings =
            settings::AppSettings::load().map_err(|_| anyhow!("Failed to load settings"))?;

        let vault_dir = PathBuf::from(&settings.vault_folder_path);
        let storage_path = vault_dir.join(format!("{storage_name}.db"));

        if db::vault_exists(&storage_path) {
            return Err(anyhow!("Storage already exists"));
        }

        db::create_new_vault(storage_path, password)
            .map_err(|e| anyhow!("Failed to create vault: {}", e))?;

        Ok(())
    })
    .await
    .map_err(|_| "Internal error".to_string())?
    .map_err(|e| {
        if e.to_string().contains("Storage already exists") {
            "Storage with this name already exists".to_string()
        } else {
            "Failed to create storage".to_string()
        }
    })
}

/// Возвращает список доступных хранилищ
#[tauri::command]
async fn populate_list() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(|| -> Result<Vec<String>, String> {
        let settings =
            settings::AppSettings::load().map_err(|_| "Failed to load settings".to_string())?;

        let vault_dir = PathBuf::from(&settings.vault_folder_path);
        std::fs::create_dir_all(&vault_dir)
            .map_err(|_| "Failed to create vault directory".to_string())?;

        let storages: Vec<String> = std::fs::read_dir(&vault_dir)
            .map_err(|_| "Failed to read vault directory".to_string())?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "db") {
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .map(|s| s.trim_end_matches(".db").to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(storages)
    })
    .await
    .map_err(|_| "Internal error".to_string())?
}

/// Открыть хранилище
#[tauri::command]
async fn open_vault(
    path: String,
    master_password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let vault = db::Vault::new(path);

    vault
        .unlock(Secret::new(Box::new(master_password)))
        .map_err(|e| e.to_string())?;

    // Сохраняем vault в состоянии
    *state.vault.lock().unwrap() = Some(vault);

    Ok(())
}

/// Закрывает сессию и блокирует хранилище
#[tauri::command]
async fn close_vault(state: State<'_, AppState>) -> Result<(), ()> {
    *state.vault.lock().unwrap() = None;
    Ok(())
}

#[tauri::command]
async fn list_services(
    state: State<'_, AppState>,
) -> Result<Vec<db::operations::ServiceSummary>, String> {
    let vault = state.vault.lock().unwrap();

    match vault.as_ref() {
        Some(v) => v.list_services().map_err(|e| e.to_string()),
        None => Err("Хранилище не открыто".to_string()),
    }
}

/// Добавляет новую запись
#[tauri::command]
async fn add_password(
    site: String,
    login: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<u64, String> {
    let vault = state.vault.lock().unwrap();

    match vault.as_ref() {
        Some(v) => v
            .add_password(&site, &login, &password)
            .map_err(|e| e.to_string()),
        None => Err("Хранилище не открыто".to_string()),
    }
}

/// Удаляет запись
#[tauri::command]
async fn delete_password(state: State<'_, AppState>, id: u64) -> Result<(), String> {
    let vault = state.vault.lock().unwrap();

    match vault.as_ref() {
        Some(v) => v.delete_password(id).map_err(|e| e.to_string()),
        None => Err("Хранилище не открыто".to_string()),
    }
}

/// Получает пароль для записи
#[tauri::command]
async fn get_password(
    state: State<'_, AppState>,
    id: u64,
) -> Result<db::operations::PasswordEntry, String> {
    let vault = state.vault.lock().unwrap();

    match vault.as_ref() {
        Some(vault) => vault.get_password(id).map_err(|e| e.to_string()),
        None => Err("Хранилище не открыто".to_string()),
    }
}
/// Проверяет существование директории
// #[tauri::command]
// async fn check_directory_exists(path: String) -> Result<bool, String> {
//     Ok(std::path::Path::new(&path).exists())
// }

/// Создает директорию
// #[tauri::command]
// async fn create_directory(path: String) -> Result<(), String> {
//     std::fs::create_dir_all(&path).map_err(|e| format!("Не удалось создать директорию: {}", e))
// }

#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> Result<Option<UpdateInfo>, String> {
    let updater = app.updater()
        .map_err(|e| format!("Не удалось инициализировать обновления: {}", e))?;
    
    match updater.check().await {
        Ok(Some(update)) => Ok(Some(UpdateInfo {
            version: update.version.clone(),
            body: update.body.clone(),
        })),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Ошибка проверки обновлений: {}", e)),
    }
}

/// Устанавливает обновление
#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    let updater = app.updater()
        .map_err(|e| format!("Не удалось инициализировать обновления: {}", e))?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            update
                .download_and_install(
                    |chunk_length, content_length| {
                        println!("Скачано: {} из {:?}", chunk_length, content_length);
                    },
                    || {
                        println!("Скачивание завершено");
                    },
                )
                .await
                .map_err(|e| format!("Ошибка установки: {}", e))?;
            
            #[cfg(not(target_os = "windows"))]
            app.restart();
            
            Ok(())
        }
        Ok(None) => Err("Нет доступных обновлений".to_string()),
        Err(e) => Err(format!("Ошибка проверки обновлений: {}", e)),
    }
}

/// Возвращает тип операционной системы («windows», «linux», «macos»)
#[tauri::command]
async fn get_os() -> Result<String, String> {
    Ok(std::env::consts::OS.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(AppState {
            vault: Arc::new(Mutex::new(None)),
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            create_vault,
            populate_list,
            open_vault,
            close_vault,
            list_services,
            check_update,
            install_update,
            get_os,
            get_password,
            add_password,
            delete_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
