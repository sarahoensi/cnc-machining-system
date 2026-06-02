// domian/machining/finishing/plannning/finishing_request.rs

use crate::domain::{
    machining::finishing::FinishingMode,
    units::{Diameter, PositiveLength},
};

#[derive(Debug, Copy, Clone)]
pub enum FinishingPlanning {
    ByCuts(u32),
    ByRadialEngagement(PositiveLength),
}

#[derive(Debug, Copy, Clone)]
pub struct FinishingRequest {
    pub mode: FinishingMode,
    pub start_diameter: Diameter,
    pub target_diameter: Diameter,
    pub planning: FinishingPlanning,
}
