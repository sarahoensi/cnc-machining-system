// src/lib.rs

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod application;
pub mod domain;
pub mod interface;
pub mod test_utils;
pub mod infrastructure;

use crate::interface::{solve_cutting_data, solve_helix, solve_right_triangle};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            solve_right_triangle,
            solve_helix,
            solve_cutting_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
