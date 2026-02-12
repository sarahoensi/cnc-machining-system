// interface/mod.rs
pub mod tauri;

pub use tauri::{
    solve_right_triangle,
    SolveRightTriangleRequest,
    SolveRightTriangleResponse,
};

pub use tauri::{solve_helix, SolveHelixRequest, SolveHelixResponse};
pub use tauri::{solve_cutting_data, SolveCuttingDataRequest, SolveCuttingDataResponse};