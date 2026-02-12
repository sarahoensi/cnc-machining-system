// interface/mod.rs

pub mod tauri;

// ---------- Right triangle ----------
pub use tauri::right_triangle::{
    solve_right_triangle,
    SolveRightTriangleRequest,
    SolveRightTriangleResponse,
};

// ---------- Helix ----------
pub use tauri::helix::{
    solve_helix,
    SolveHelixRequest,
    SolveHelixResponse,
};

// ---------- Cutting data ----------
pub use tauri::cutting_data::{
    solve_cutting_data,
    SolveCuttingDataRequest,
    SolveCuttingDataResponse,
};

// ---------- Finishing ----------
pub use tauri::finishing::{
    generate_finishing_plan,
    register_finishing_measurement,
    GenerateFinishingPlanRequest,
    RegisterFinishingMeasurementRequest,
    FinishingExecutionResponse,
};
