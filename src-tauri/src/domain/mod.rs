// domain/mod.rs
pub mod units;
mod geometry;
mod machining_physics;
mod machining_strategy;
pub mod error;



// --------------------------------
// Geometry
// --------------------------------
pub use geometry::{
    Circle,
    Helix,
    HelixAngle,
    HelixMode,
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
    MachiningPhysicsError
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
   };
