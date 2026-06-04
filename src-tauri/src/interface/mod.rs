// interface/mod.rs

//! Public interface surface for backend features.
//!
//! Each module represents a frontend feature entrypoint.

pub mod tauri;

pub use tauri::{cutting_data, cylinder_weight, finishing, helix, right_triangle, tolerance};
