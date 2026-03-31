// src-tauri/src/modules/file_utils.rs
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn validate_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        Err(format!("路径不存在: {}", path.display()))
    } else if !path.is_dir() {
        Err(format!("不是目录: {}", path.display()))
    } else {
        Ok(())
    }
}

pub fn collect_files_with_unmatched(
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

pub fn is_extension_match(ext: &str, extensions: &[String]) -> bool {
    extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
}

pub fn extract_extension(file_name: &str) -> Option<String> {
    file_name
        .rfind('.')
        .map(|pos| file_name[pos..].to_lowercase())
}

pub fn extract_name_without_ext(file_name: &str) -> String {
    file_name
        .rfind('.')
        .map(|pos| &file_name[..pos])
        .unwrap_or(file_name)
        .to_string()
}

pub fn name_matches(file1: &str, file2: &str) -> bool {
    extract_name_without_ext(file1) == extract_name_without_ext(file2)
}

pub fn copy_file(src: &Path, dst: &Path) -> bool {
    fs::copy(src, dst).is_ok()
}

pub fn move_file(src: &Path, dst: &Path) -> bool {
    if fs::rename(src, dst).is_ok() {
        true
    } else if fs::copy(src, dst).is_ok() {
        fs::remove_file(src).is_ok()
    } else {
        false
    }
}

pub fn make_unique_path(path: &Path) -> PathBuf {
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