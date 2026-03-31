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
    _backup_strategy: String,
    dry_run: bool,
) -> Result<OrganizeResult, String> {
    let main_path = PathBuf::from(&main_folder);
    let source_path = main_path.join(&source_folder);
    let backup_path = main_path.join(&backup_folder);

    validate_path(&source_path)?;
    validate_path(&backup_path)?;

    let mode = mode.to_lowercase();
    let folder_name = if mode == "move" { "move" } else { "copy" };
    let main_folder_name = main_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("main");
    let target_folder = main_path.join(folder_name).join(main_folder_name);
    if !dry_run {
        fs::create_dir_all(&target_folder)
            .map_err(|e| format!("创建{}文件夹失败: {}", target_folder.display(), e))?;
    }
    let output_prefix = format!("{}/{}", folder_name, main_folder_name);

    let (source_files, mut source_skipped) = collect_files_with_unmatched(&source_path, &source_extensions)?;
    let (backup_files, mut backup_skipped) = collect_files_with_unmatched(&backup_path, &backup_extensions)?;

    let mut moved_files = Vec::new();
    let mut skipped_files = Vec::new();
    let mut created_folders = Vec::new();

    skipped_files.append(&mut source_skipped);
    skipped_files.append(&mut backup_skipped);

    let mut matched_source_names = std::collections::HashSet::new();
    let mut matched_backup_names = std::collections::HashSet::new();

    for (source_name, source_file_path) in source_files.iter() {
        let backup_match = backup_files
            .iter()
            .find(|(backup_name, _)| !matched_backup_names.contains(*backup_name) && name_matches(source_name, backup_name));

        if let Some((backup_name, backup_file_path)) = backup_match {
            matched_source_names.insert(source_name.clone());
            matched_backup_names.insert(backup_name.clone());

            skipped_files.push(format!("匹配文件，处理源文件和备份文件：{} <-> {}", source_name, backup_name));

            if !process_single_file(
                &output_prefix,
                &target_folder,
                source_name,
                source_file_path,
                &mode,
                dry_run,
                &mut moved_files,
                &mut skipped_files,
                &mut created_folders,
            ) {
                skipped_files.push(format!("无法{}文件: {}", if mode == "move" { "移动" } else { "复制" }, source_name));
            }

            if !process_single_file(
                &output_prefix,
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
    }

    for (source_name, _) in source_files.iter() {
        if !matched_source_names.contains(source_name) {
            skipped_files.push(format!("源目录文件未在备目录中找到匹配: {}", source_name));
        }
    }

    for (backup_name, _) in backup_files.iter() {
        if !matched_backup_names.contains(backup_name) {
            skipped_files.push(format!("备目录文件未在源目录中找到匹配: {}", backup_name));
        }
    }

    Ok(OrganizeResult {
        moved_files,
        skipped_files,
        created_folders,
    })
}

fn process_single_file(
    output_prefix: &str,
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
        let ext_name = ext.trim_start_matches('.');
        let ext_folder = target_folder.join(ext_name);

        if !ext_folder.exists() {
            if dry_run {
                created_folders.push(format!("{}/{}", output_prefix, ext_name));
            } else {
                if let Err(e) = fs::create_dir_all(&ext_folder) {
                    skipped_files.push(format!("无法创建文件夹 {}: {}", ext_name, e));
                    return false;
                }
                created_folders.push(format!("{}/{}", output_prefix, ext_name));
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
                moved_files.push(format!("{}/{}/{}", output_prefix, ext_name, target_name));
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