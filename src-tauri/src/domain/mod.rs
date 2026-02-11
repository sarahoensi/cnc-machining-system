// domain/mod.rs
mod units;
mod geometry;
mod machining_physics;
mod machining_strategy;

// --------------------------------
// Units
// --------------------------------
pub use units::{
    Length,
    Diameter,
    Radius,
    Angle,
    CuttingSpeed,
    FeedRate,
    Rpm,
    ChipLoad,
    Pitch
};

pub use units::UnitError;

// --------------------------------
// Geometry
// --------------------------------
pub use geometry::{
    Circle,
    Helix,
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
