mod modules;

use modules::file_selector::*;
use modules::file_processor::*;

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
