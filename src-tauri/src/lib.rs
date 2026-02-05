// lib.rs

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod domain;
pub mod application;
pub mod interface;


use interface::features::cutting_data::tauri_commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            calculate_partial_cutting_data,
            calculate_full_cutting_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}