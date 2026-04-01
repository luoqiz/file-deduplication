// src-tauri/src/modules/file_selector.rs
use std::fs;
use std::path::Path;

#[tauri::command]
pub async fn select_folder() -> Result<String, String> {
    let folder = tokio::task::spawn_blocking(|| {
        rfd::FileDialog::new().pick_folder()
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
    .ok_or_else(|| "未选择文件夹".to_string())?;

    folder
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "文件夹路径无效".to_string())
}

#[tauri::command]
pub async fn list_folder(path: String) -> Result<Vec<String>, String> {
    let dir_path = Path::new(&path);

    if !dir_path.exists() {
        return Err(format!("路径不存在: {}", path));
    }

    if !dir_path.is_dir() {
        return Err(format!("不是目录: {}", path));
    }

    let mut folders = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        folders.push(name.to_string());
                    }
                }
            }
        }
    }

    folders.sort();
    Ok(folders)
}