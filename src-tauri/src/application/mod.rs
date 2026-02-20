//! Application layer entry point for machining workflows.
//!
//! This module exposes orchestration use cases used by external interfaces
//! (such as UI commands) and keeps domain rules inside the domain layer.
//! It wires geometry solving, helix solving, cutting-data completion, and
//! finishing lifecycle workflows into a consistent application API.
//!
//! Domain behavior is delegated to domain services and aggregates; this layer
//! is responsible for input/output shaping and use-case coordination.

// application/mod.rs
 mod right_triangle;
 mod helix;
 mod shared;
 mod cutting_data;
 pub mod finishing;

pub use shared::ApplicationError;

pub use right_triangle::solve_right_triangle_use_case::SolveRightTriangleUseCase;
pub use right_triangle::dto::{SolveRightTriangleInput, SolveRightTriangleOutput};

pub use helix::solve_helix_use_case::SolveHelixUseCase;
pub use helix::dto::{SolveHelixInput, SolveHelixOutput};

pub use cutting_data::dto::{SolveCuttingDataInput, SolveCuttingDataOutput};
pub use cutting_data::solve_cutting_data_use_case::SolveCuttingDataUseCase;
