// src/lib.rs

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod application;
pub mod domain;
pub mod interface;
pub mod test_utils;

use std::sync::Mutex;
use tauri::Manager;

use crate::application::JsonCylinderMaterialRepository;
use crate::domain::machining::finishing::FinishingExecution;
use crate::interface::{
    cutting_data::solve_cutting_data,
    cylinder_weight::{
        create_cylinder_material, delete_cylinder_material, export_cylinder_materials,
        import_cylinder_materials, list_cylinder_materials, solve_cylinder_weight,
        update_cylinder_material,
    },
    finishing::{generate_finishing_plan, register_finishing_measurement},
    helix::solve_helix,
    right_triangle::solve_right_triangle,
    thread::{list_thread_options, solve_thread},
    tolerance::{calculate_iso286_fit, list_iso286_tolerance_options, lookup_iso286_tolerance},
};

pub struct AppState {
    pub finishing_execution: Mutex<Option<FinishingExecution>>,
    pub cylinder_material_repository: Mutex<JsonCylinderMaterialRepository>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let mut path = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("failed to resolve app data dir: {e}"))?;
            path.push("cylinder_materials.json");

            let repo = JsonCylinderMaterialRepository::load_or_initialize(path)
                .map_err(|e| format!("failed to initialize cylinder material repository: {e}"))?;

            app.manage(AppState {
                finishing_execution: std::sync::Mutex::new(None),
                cylinder_material_repository: std::sync::Mutex::new(repo),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // right triangle
            solve_right_triangle,
            // helix
            solve_helix,
            // cutting_data
            solve_cutting_data,
            // tolerances
            calculate_iso286_fit,
            lookup_iso286_tolerance,
            list_iso286_tolerance_options,
            // threads
            list_thread_options,
            solve_thread,
            // cylinder_weight
            list_cylinder_materials,
            create_cylinder_material,
            update_cylinder_material,
            delete_cylinder_material,
            import_cylinder_materials,
            export_cylinder_materials,
            solve_cylinder_weight,
            // finishing
            generate_finishing_plan,
            register_finishing_measurement,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
