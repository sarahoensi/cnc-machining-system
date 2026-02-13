//! Tauri commands for helix-solving workflows.
//!
//! This feature area supports frontend machining setup screens that compute
//! helix parameters by pitch or angle through application-layer orchestration.

// interface/tauri/helix/mod.rs

mod command;
mod request;
mod response;
mod mapping;

pub use command::solve_helix;
pub use request::{SolveHelixRequest, HelixMode};
pub use response::SolveHelixResponse;
