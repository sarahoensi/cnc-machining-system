// interface/tauri/mod.rs

pub mod cutting_data;
pub mod finishing;
pub mod helix;

pub mod right_triangle;

pub use right_triangle::{
    solve_right_triangle,
    SolveRightTriangleRequest,
    SolveRightTriangleResponse,
};


pub use cutting_data::{solve_cutting_data, SolveCuttingDataRequest, SolveCuttingDataResponse};
pub use finishing::{
     generate_finishing_plan, register_finishing_measurement,
};
pub use helix::{solve_helix, SolveHelixRequest, SolveHelixResponse};
