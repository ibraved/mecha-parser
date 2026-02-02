#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mecha_parser::commands;

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .manage(commands::TrackerController::default())
    .invoke_handler(tauri::generate_handler![
      commands::start_tracking,
      commands::stop_tracking,
      commands::get_status,
      commands::get_config,
      commands::set_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

