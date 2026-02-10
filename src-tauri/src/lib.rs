// src/lib.rs

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod domain;
pub mod test_utils;
pub mod application;
pub mod interface;

use crate::interface::tauri::solve_right_triangle;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            solve_right_triangle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}



    
