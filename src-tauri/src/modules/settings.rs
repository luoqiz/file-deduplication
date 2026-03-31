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
    pub source_folder: String,
    pub backup_folder: String,
    pub source_extensions: Vec<String>,
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