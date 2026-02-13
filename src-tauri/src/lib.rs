// src/lib.rs

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod test_utils;

use crate::interface::{
    generate_finishing_plan, register_finishing_measurement, solve_cutting_data, solve_helix,
    solve_right_triangle,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // right triangle
            solve_right_triangle,

            // helix
            solve_helix,

            // cutting_data
            solve_cutting_data,

            // finishing
            generate_finishing_plan,
            register_finishing_measurement,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
