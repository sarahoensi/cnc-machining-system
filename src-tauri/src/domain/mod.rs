// domain/mod.rs
pub mod units;
mod geometry;
mod machining_physics;
mod machining_strategy;



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
};

// --------------------------------
// Machining Physics
// --------------------------------
pub use machining_physics::{
    Tool,
    ToothCount,

    ChipLoadCalculator,
    FeedRateCalculator,
    SpindleSpeedCalculator,

    CuttingInput,
    CuttingResult,
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
