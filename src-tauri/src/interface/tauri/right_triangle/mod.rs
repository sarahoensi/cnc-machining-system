//! Tauri commands for right-triangle machining geometry workflows.
//!
//! This feature area supports frontend geometry setup screens and forwards
//! requests to the right-triangle application use case.

// interface/tauri/right_triangle/mod.rs

// interface/tauri/right_triangle/mod.rs

mod command;
mod request;
mod response;
mod mapping;

pub use command::solve_right_triangle;
pub use request::SolveRightTriangleRequest;
pub use response::SolveRightTriangleResponse;
