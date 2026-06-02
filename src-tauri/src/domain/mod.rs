// domain/mod.rs
mod geometry;
pub mod machining;
pub mod units;
//mod machining_strategy;
mod error;

pub use error::DomainError;

// --------------------------------
// Geometry
// --------------------------------
pub use geometry::{
    Circle,
    //RightTriangleSolver,
    GeometryError,
    Helix,
    HelixError,
    HelixMode,
    //EffectiveDiameter,
    RightTriangle,
    RightTriangleError,
};

// --------------------------------
// Machining Physics
// --------------------------------
/*
pub use machining_physics::{
    CuttingParameters,
    MachiningSolver,
    MachiningPhysicsError,
Tool,

};
 */

// --------------------------------
// Machining Strategy
// --------------------------------
/*
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
*/
