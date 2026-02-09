// domain/machining_strategy/finishing/finishing_request.rs

use crate::domain::units::{Diameter, Length};


use crate::domain::machining_strategy::finishing::{
    FinishingMode,
    
};


/// Defines how finishing plan should be generated.
#[derive(Debug, Copy, Clone)]
pub enum FinishingPlanning {
    /// User specifies number of cuts directly
    ByCuts(u32),

    /// User specifies radial engagement (ae)
    ByRadialEngagement(Length),
}

/// Domain-level request for generating a finishing plan.
#[derive(Debug, Copy, Clone)]
pub struct FinishingRequest {
    pub mode: FinishingMode,
    pub start_diameter: Diameter,
    pub target_diameter: Diameter,
    pub planning: FinishingPlanning,
}
