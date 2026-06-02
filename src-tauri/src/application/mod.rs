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
mod cutting_data;
mod cylinder_weight;
pub mod finishing;
mod helix;
mod right_triangle;
pub mod shared;

pub use shared::{ApplicationError, ValidationErrors};

pub use right_triangle::dto::{SolveRightTriangleInput, SolveRightTriangleOutput};
pub use right_triangle::solve_right_triangle_use_case::SolveRightTriangleUseCase;

pub use helix::dto::{SolveHelixInput, SolveHelixOutput};
pub use helix::solve_helix_use_case::SolveHelixUseCase;

pub use cutting_data::dto::{SolveCuttingDataInput, SolveCuttingDataOutput};
pub use cutting_data::solve_cutting_data_use_case::SolveCuttingDataUseCase;

pub use cylinder_weight::{
    CreateCylinderMaterialInput, CreateCylinderMaterialUseCase, CylinderMaterialOutput,
    CylinderMaterialRecord, CylinderMaterialRepository, DeleteCylinderMaterialInput,
    DeleteCylinderMaterialUseCase, ExportCylinderMaterialRow, ExportCylinderMaterialsOutput,
    ExportCylinderMaterialsUseCase, ImportAddedMaterialRow, ImportCylinderMaterialsInput,
    ImportCylinderMaterialsOutput, ImportCylinderMaterialsUseCase, ImportSkippedMaterialRow,
    JsonCylinderMaterialRepository, ListCylinderMaterialsUseCase, SolveCylinderWeightInput,
    SolveCylinderWeightOutput, SolveCylinderWeightUseCase, UpdateCylinderMaterialInput,
    UpdateCylinderMaterialUseCase,
};
