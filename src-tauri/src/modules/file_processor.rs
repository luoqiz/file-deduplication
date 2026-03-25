// src-tauri/src/modules/file_processor.rs
use std::fs;
use std::path::{Path, PathBuf};

use crate::modules::file_utils::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OrganizeResult {
    pub moved_files: Vec<String>,
    pub skipped_files: Vec<String>,
    pub created_folders: Vec<String>,
}

#[tauri::command]
pub async fn process_files(
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