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
    mode: String,
    backup_strategy: String,
    dry_run: bool,
) -> Result<OrganizeResult, String> {
   let main_path = PathBuf::from(&main_folder);
    let source_path = main_path.join(&source_folder);
    let backup_path = main_path.join(&backup_folder);

    validate_path(&source_path)?;
    validate_path(&backup_path)?;

    let mode = mode.to_lowercase();
    let strategy = backup_strategy.to_lowercase();
    let folder_name = if mode == "move" { "move" } else { "copy" };
    let target_folder = main_path.join(folder_name);
    if !dry_run {
        fs::create_dir_all(&target_folder)
            .map_err(|e| format!("创建{}文件夹失败: {}", folder_name, e))?;
    }

    let (source_files, mut source_skipped) = collect_files_with_unmatched(&source_path, &source_extensions)?;
    let (backup_files, mut backup_skipped) = collect_files_with_unmatched(&backup_path, &backup_extensions)?;

    let mut moved_files = Vec::new();
    let mut skipped_files = Vec::new();
    let mut created_folders = Vec::new();

    skipped_files.append(&mut source_skipped);
    skipped_files.append(&mut backup_skipped);

    let mut matched_backup_names = std::collections::HashSet::new();

    for (file_name, source_file_path) in source_files.iter() {
        let backup_match = backup_files
            .iter()
            .find(|(backup_name, _)| !matched_backup_names.contains(*backup_name) && name_matches(file_name, backup_name));

        if let Some((backup_name, backup_file_path)) = backup_match {
            matched_backup_names.insert(backup_name.clone());

            let source_is_preferred = choose_preferred_by_strategy(source_file_path, backup_file_path, &strategy);
            if source_is_preferred {
                skipped_files.push(format!("选中源文件，跳过备份：{} <-> {}", file_name, backup_name));
                if !process_single_file(
                    folder_name,
                    &target_folder,
                    file_name,
                    source_file_path,
                    &mode,
                    dry_run,
                    &mut moved_files,
                    &mut skipped_files,
                    &mut created_folders,
                ) {
                    skipped_files.push(format!("无法{}文件: {}", if mode == "move" { "移动" } else { "复制" }, file_name));
                }
            } else {
                skipped_files.push(format!("选中备份文件，跳过源：{} <-> {}", file_name, backup_name));
                if !process_single_file(
                    folder_name,
                    &target_folder,
                    backup_name,
                    backup_file_path,
                    &mode,
                    dry_run,
                    &mut moved_files,
                    &mut skipped_files,
                    &mut created_folders,
                ) {
                    skipped_files.push(format!("无法{}文件: {}", if mode == "move" { "移动" } else { "复制" }, backup_name));
                }
            }

            continue;
        }

        // 未匹配备份，直接处理源
        if !process_single_file(
            folder_name,
            &target_folder,
            file_name,
            source_file_path,
            &mode,
            dry_run,
            &mut moved_files,
            &mut skipped_files,
            &mut created_folders,
        ) {
            skipped_files.push(format!("无法{}文件: {}", if mode == "move" { "移动" } else { "复制" }, file_name));
        }
    }

    // 处理没有匹配到源的备份文件
    for (backup_name, backup_file_path) in backup_files.iter() {
        if matched_backup_names.contains(backup_name) {
            continue;
        }

        if !process_single_file(
            folder_name,
            &target_folder,
            backup_name,
            backup_file_path,
            &mode,
            dry_run,
            &mut moved_files,
            &mut skipped_files,
            &mut created_folders,
        ) {
            skipped_files.push(format!("无法{}文件: {}", if mode == "move" { "移动" } else { "复制" }, backup_name));
        }
    }

    Ok(OrganizeResult {
        moved_files,
        skipped_files,
        created_folders,
    })
}

fn process_single_file(
    folder_name: &str,
    target_folder: &Path,
    file_name: &str,
    file_path: &Path,
    mode: &str,
    dry_run: bool,
    moved_files: &mut Vec<String>,
    skipped_files: &mut Vec<String>,
    created_folders: &mut Vec<String>,
) -> bool {
    if let Some(ext) = extract_extension(file_name) {
        let ext_folder = target_folder.join(&ext);

        if !ext_folder.exists() {
            if dry_run {
                created_folders.push(format!("{}/{}", folder_name, ext));
            } else {
                if let Err(e) = fs::create_dir_all(&ext_folder) {
                    skipped_files.push(format!("无法创建文件夹 {}: {}", ext, e));
                    return false;
                }
                created_folders.push(format!("{}/{}", folder_name, ext));
            }
        }

        let target = make_unique_path(&ext_folder.join(file_name));
        let ok = if dry_run {
            true // 在dry_run模式下，假设操作成功
        } else {
            if mode == "move" {
                move_file(file_path, &target)
            } else {
                copy_file(file_path, &target)
            }
        };

        if ok {
            if let Some(target_name) = target.file_name().and_then(|n| n.to_str()) {
                moved_files.push(format!("{}/{}/{}", folder_name, ext, target_name));
            }
            true
        } else {
            false
        }
    } else {
        skipped_files.push(format!("文件无扩展名，已跳过: {}", file_name));
        false
    }
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

fn choose_preferred_by_strategy(source_path: &Path, backup_path: &Path, strategy: &str) -> bool {
    if strategy == "source" {
        return true;
    }
    if strategy == "backup" {
        return false;
    }

    let source_meta = source_path.metadata();
    let backup_meta = backup_path.metadata();

    if let (Ok(source_meta), Ok(backup_meta)) = (source_meta, backup_meta) {
        let source_size = source_meta.len();
        let backup_size = backup_meta.len();
        let source_modified = source_meta.modified().ok();
        let backup_modified = backup_meta.modified().ok();

        if strategy == "largest" {
            if source_size != backup_size {
                return source_size > backup_size;
            }
        }

        if strategy == "newest" {
            if let (Some(source_time), Some(backup_time)) = (source_modified, backup_modified) {
                return source_time >= backup_time;
            }
        }

        if source_size != backup_size {
            return source_size > backup_size;
        }

        if let (Some(source_time), Some(backup_time)) = (source_modified, backup_modified) {
            return source_time >= backup_time;
        }
    }

    // 默认以来源优先
    true
}

fn collect_files_with_unmatched(
    dir_path: &Path,
    extensions: &[String],
) -> Result<(HashMap<String, PathBuf>, Vec<String>), String> {
    let mut files = HashMap::new();
    let mut skipped = Vec::new();

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
                                } else {
                                    skipped.push(format!("文件后缀不匹配: {}", file_name));
                                }
                            }
                        } else {
                            skipped.push(format!("文件无后缀，已跳过: {}", file_name));
                        }
                    }
                }
            }
        }
    }

    Ok((files, skipped))
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

fn move_file(src: &Path, dst: &Path) -> bool {
    if fs::rename(src, dst).is_ok() {
        true
    } else if fs::copy(src, dst).is_ok() {
        fs::remove_file(src).is_ok()
    } else {
        false
    }
}

fn make_unique_path(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }

    let mut idx = 1;
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = path.extension().and_then(|s| s.to_str());

    loop {
        let candidate = if let Some(ext_val) = ext {
            path.with_file_name(format!("{}({}).{}", stem, idx, ext_val))
        } else {
            path.with_file_name(format!("{}({})", stem, idx))
        };

        if !candidate.exists() {
            return candidate;
        }

        idx += 1;
    }
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
