// interface/tauri/mod.rs

//! Tauri interface boundary for CNC machining workflows.
//!
//! This module groups command surfaces exposed to the frontend and routes UI
//! requests into application-layer use cases for geometry, helix, finishing,
//! and cutting-data workflows.

pub mod cutting_data;
pub mod cylinder_weight;
pub mod finishing;
pub mod helix;
pub mod right_triangle;
pub mod tolerance;

mod error;
