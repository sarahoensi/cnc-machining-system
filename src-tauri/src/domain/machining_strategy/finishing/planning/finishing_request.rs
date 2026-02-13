// domain/machining_strategy/finishing/finishing_request.rs

use crate::domain::{
    units::{Diameter, Length},
    FinishingMode,
};



/// Specifies how a finishing plan should be generated.
///
/// Determines whether the plan is derived from a fixed number
/// of cuts or from a desired radial engagement.
#[derive(Debug, Copy, Clone)]
pub enum FinishingPlanning {
    /// The number of finishing cuts is explicitly defined by the user.
    ByCuts(u32),

    /// The number of finishing cuts is derived from radial engagement (ae).
    ///
    /// Radial engagement represents the material removed per pass.
    ByRadialEngagement(Length),
}

/// Domain-level request used to generate a finishing plan.
///
/// This struct describes the machining intent and contains
/// all required inputs for the finishing planner.
#[derive(Debug, Copy, Clone)]
pub struct FinishingRequest {
    /// Finishing mode (inner or outer diameter).
    pub mode: FinishingMode,

    /// Starting diameter before finishing.
    pub start_diameter: Diameter,

    /// Target diameter after finishing.
    pub target_diameter: Diameter,

    /// Planning strategy describing how cuts are determined.
    pub planning: FinishingPlanning,
}