use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub vault_folder_path: String,
    pub theme: String, // "latte", "frappe", "macchiato", "mocha"
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            vault_folder_path: String::new(),
            theme: "mocha".to_string(),
        }
    }
}

impl AppSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_vault_folder_path(&self) -> PathBuf {
        PathBuf::from(&self.vault_folder_path)
    }

    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("ru", "CEBikol", "nopeekpanda")
            .context("Failed to determine project directories (OS-specific paths)")?;

        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)
            .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

        Ok(config_dir.join("settings.json"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            let settings = Self::default();
            settings.save().context("Failed to save default settings")?;
            return Ok(settings);
        }

        let contents = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

        let settings = serde_json::from_str(&contents)
            .context("Invalid JSON in config file. Delete settings.json to reset")?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let json =
            serde_json::to_string_pretty(self).context("Failed to serialize settings to JSON")?;

        fs::write(&config_path, json)
            .with_context(|| format!("Failed to write config to: {:?}", config_path))?;

        Ok(())
    }

    pub fn update_vault_path(&mut self, new_path: String) -> Result<()> {
        self.vault_folder_path = new_path;
        self.save()
    }
}
