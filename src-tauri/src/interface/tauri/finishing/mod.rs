//! Tauri commands for finishing workflow lifecycle management.
//!
//! This feature area supports frontend operations to:
//! - generate finishing plans
//! - register measured step feedback
//!
//! It orchestrates finishing application use cases and exposes the serialized
//! boundary contract used by UI clients.

// interface/tauri/finishing/mod.rs
mod request;
mod response;
mod mapping;
mod command;

// Public command surface
pub use command::*;

// Public DTO surface
pub use request::*;
pub use response::*;