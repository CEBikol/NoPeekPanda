// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Загружаем настройки при запуске и выводим информацию
    match nopeekpanda_lib::utils::settings::AppSettings::load() {
        Ok(settings) => {
            println!("Загружены настройки приложения:");
            println!("Тема: {}", settings.theme);
            println!("Папка хранилища: {}", settings.vault_folder_path);
            println!("Запуск приложения...");
        }
        Err(e) => {
            eprintln!("Не удалось загрузить настройки: {}", e);
            println!("Будут использованы настройки по умолчанию");
        }
    }

    nopeekpanda_lib::run()
}
