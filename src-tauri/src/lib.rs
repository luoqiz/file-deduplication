use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizeResult {
    pub moved_files: Vec<String>,
    pub skipped_files: Vec<String>,
    pub created_folders: Vec<String>,
}

#[tauri::command]
async fn select_folder() -> Result<String, String> {
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
async fn list_folder(path: String) -> Result<Vec<String>, String> {
   let dir_path = PathBuf::from(&path);

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



#[tauri::command]
async fn process_files(
      main_folder: String,
    source_folder: String,
    backup_folder: String,
    source_extensions: Vec<String>,
    backup_extensions: Vec<String>,
) -> Result<OrganizeResult, String> {
   let main_path = PathBuf::from(&main_folder);
    let source_path = main_path.join(&source_folder);
    let backup_path = main_path.join(&backup_folder);

    validate_path(&source_path)?;
    validate_path(&backup_path)?;

    let move_folder = main_path.join("move");
    fs::create_dir_all(&move_folder)
        .map_err(|e| format!("创建move文件夹失败: {}", e))?;

    let source_files = collect_files(&source_path, &source_extensions)?;
    let backup_files = collect_files(&backup_path, &backup_extensions)?;

    let mut moved_files = Vec::new();
    let mut skipped_files = Vec::new();
    let mut created_folders = Vec::new();

    for (file_name, source_file_path) in source_files.iter() {
        if let Some(ext) = extract_extension(file_name) {
            let ext_folder = move_folder.join(&ext);

            if !ext_folder.exists() {
                if let Err(e) = fs::create_dir_all(&ext_folder) {
                    skipped_files.push(format!("无法创建文件夹 {}: {}", ext, e));
                    continue;
                }
                created_folders.push(format!("move/{}", ext));
            }

            if copy_file(source_file_path, &ext_folder.join(file_name)) {
                moved_files.push(format!("move/{}/{}", ext, file_name));
            } else {
                skipped_files.push(format!("无法复制文件: {}", file_name));
            }

            for (backup_file_name, backup_file_path) in backup_files.iter() {
                if name_matches(file_name, backup_file_name) {
                    if copy_file(backup_file_path, &ext_folder.join(backup_file_name)) {
                        moved_files.push(format!("move/{}/{}", ext, backup_file_name));
                    } else {
                        skipped_files.push(format!("无法复制备用文件: {}", backup_file_name));
                    }
                }
            }
        }
    }

    Ok(OrganizeResult {
        moved_files,
        skipped_files,
        created_folders,
    })
}


fn validate_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        Err(format!("路径不存在: {}", path.display()))
    } else if !path.is_dir() {
        Err(format!("不是目录: {}", path.display()))
    } else {
        Ok(())
    }
}

fn collect_files(
    dir_path: &Path,
    extensions: &[String],
) -> Result<HashMap<String, PathBuf>, String> {
    let mut files = HashMap::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        let file_path = entry.path();
                        if let Some(ext) = file_path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                let ext_with_dot = format!(".{}", ext_str).to_lowercase();
                                if is_extension_match(&ext_with_dot, extensions) {
                                    files.insert(file_name.to_string(), file_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}

fn is_extension_match(ext: &str, extensions: &[String]) -> bool {
    extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
}

fn extract_extension(file_name: &str) -> Option<String> {
    file_name
        .rfind('.')
        .map(|pos| file_name[pos..].to_lowercase())
}

fn extract_name_without_ext(file_name: &str) -> String {
    file_name
        .rfind('.')
        .map(|pos| &file_name[..pos])
        .unwrap_or(file_name)
        .to_string()
}

fn name_matches(file1: &str, file2: &str) -> bool {
    extract_name_without_ext(file1) == extract_name_without_ext(file2)
}

fn copy_file(src: &Path, dst: &Path) -> bool {
    fs::copy(src, dst).is_ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            select_folder,
            list_folder,
            process_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
