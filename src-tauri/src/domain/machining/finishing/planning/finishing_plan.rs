// domain/machining/finishing/planning/finishing_plan.rs

use crate::domain::{
    machining::finishing::FinishingMode,
    units::{Diameter, PositiveLength},
};

#[derive(Debug, Copy, Clone)]
pub struct FinishingPlan {
    mode: FinishingMode,
    start: Diameter,
    target: Diameter,
    cuts: u32,
    expected_step: PositiveLength,
}

impl FinishingPlan {
    pub(crate) fn new(
        mode: FinishingMode,
        start: Diameter,
        target: Diameter,
        cuts: u32,
        expected_step: PositiveLength,
    ) -> Self {
        Self {
            mode,
            start,
            target,
            cuts,
            expected_step,
        }
    }

    pub fn mode(&self) -> FinishingMode {
        self.mode
    }

    pub fn start(&self) -> Diameter {
        self.start
    }

    pub fn target(&self) -> Diameter {
        self.target
    }

    pub fn cuts(&self) -> u32 {
        self.cuts
    }

    pub fn expected_step(&self) -> PositiveLength {
        self.expected_step
    }
}
