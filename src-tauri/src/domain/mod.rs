// domain/mod.rs
pub mod units;
mod geometry;
mod machining_physics;
mod machining_strategy;
mod error;

pub use error::DomainError;

// --------------------------------
// Geometry
// --------------------------------
pub use geometry::{
    Circle,
    Helix,
    HelixAngle,
    HelixMode,
    HelixError,
    EffectiveDiameter,
    RightTriangle,
    RightTriangleSolver,
    GeometryError,
    RightTriangleError
};

// --------------------------------
// Machining Physics
// --------------------------------
pub use machining_physics::{
    CuttingParameters,
    MachiningSolver,
    MachiningPhysicsError,
Tool,
ToothCount
};

// --------------------------------
// Machining Strategy
// --------------------------------
pub use machining_strategy::{
    FinishingPlanner,
    FinishingExecution,
    FinishingPlan,
    FinishingStep,
    FinishingMode,
    FinishingRequest,
    FinishingPlanning,
    StrategyError,
    FinishingExecutionId,
    FinishingExecutionRepository,
    FinishingRepositoryError,
   };
