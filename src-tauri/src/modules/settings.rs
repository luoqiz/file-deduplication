use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::command;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub selected_folder: String,
    pub operation_mode: String,
    pub backup_strategy: String,
    pub dry_run: bool,
    #[serde(default)]
    pub include_subdirectories: bool,
    #[serde(default, alias = "sourceFolder", alias = "source_folder")]
    pub source_folders: Vec<String>,
    #[serde(default, alias = "backupFolder", alias = "backup_folder")]
    pub backup_folders: Vec<String>,
    #[serde(default, alias = "sourceExtensions", alias = "source_extensions")]
    pub source_extensions: Vec<String>,
    #[serde(default, alias = "backupExtensions", alias = "backup_extensions")]
    pub backup_extensions: Vec<String>,
}

#[command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    let toml_string = toml::to_string(&settings).map_err(|e| e.to_string())?;
    let path = Path::new("setting.toml");
    fs::write(path, toml_string).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn load_settings() -> Result<Settings, String> {
    let path = Path::new("setting.toml");
    if !path.exists() {
        return Err("Settings file not found".to_string());
    }
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let settings: Settings = toml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(settings)
}