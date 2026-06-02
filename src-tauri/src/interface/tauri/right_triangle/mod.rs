//! Tauri commands for right-triangle machining geometry workflows.
//!
//! This feature area supports frontend geometry setup screens and forwards
//! requests to the right-triangle application use case.

// interface/tauri/right_triangle/mod.rs

// interface/tauri/right_triangle/mod.rs

mod command;
mod mapping;
mod request;
mod response;

pub use command::*;
pub use request::*;
pub use response::*;
