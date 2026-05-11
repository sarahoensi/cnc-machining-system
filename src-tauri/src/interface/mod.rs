// interface/mod.rs

//! Public interface surface for backend features.
//!
//! Each module represents a frontend feature entrypoint.

pub mod tauri;

pub use tauri::{
    right_triangle,
    helix,
    cutting_data,
    cylinder_weight,
    finishing,
};
