// domain/machining_strategy/finishing/finishing_plan.rs

use crate::domain::{Diameter, Length};

use super::FinishingMode;

/// Static plan: start/target + number of cuts and expected per-step delta.
#[derive(Debug, Copy, Clone)]
pub struct FinishingPlan {
    mode: FinishingMode,
    start: Diameter,
    target: Diameter,
    cuts: u32,
    expected_step: Length, // always positive magnitude
}

impl FinishingPlan {
    pub fn mode(&self) -> FinishingMode { self.mode }
    pub fn start(&self) -> Diameter { self.start }
    pub fn target(&self) -> Diameter { self.target }
    pub fn cuts(&self) -> u32 { self.cuts }
    pub fn expected_step(&self) -> Length { self.expected_step }

    pub fn direction_sign(&self) -> f64 {
        self.mode.direction_sign()
    }
}
impl FinishingPlan {

    pub(crate) fn new(
        mode: FinishingMode,
        start: Diameter,
        target: Diameter,
        cuts: u32,
        expected_step: Length,
    ) -> Self {
        Self {
            mode,
            start,
            target,
            cuts,
            expected_step,
        }
    }

}
