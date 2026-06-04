//! Tauri commands for cutting-data calculation workflows.
//!
//! This feature area supports frontend forms that complete machining cutting
//! parameters by invoking the cutting-data application use case.
//!
//! It acts as a UI/backend boundary by exposing request/response DTOs and a
//! command function with string-based error propagation.

// interface/tauri/cutting_data/mod.rs

mod command;
mod mapping;
mod request;
mod response;

pub use command::*;
pub use request::*;
pub use response::*;
