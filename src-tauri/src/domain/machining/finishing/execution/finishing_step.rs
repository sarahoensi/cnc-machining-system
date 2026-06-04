// domain/machining/finishing/execution/finishhing_step.rs

use crate::domain::units::{Diameter, PositiveLength};

#[derive(Debug, Copy, Clone)]
pub struct FinishingStep {
    index: u32,
    start: Diameter,
    planned_delta: PositiveLength,
    planned_end: Diameter,
    measurement: Option<Diameter>,
}

impl FinishingStep {
    pub fn new(
        index: u32,
        start: Diameter,
        planned_delta: PositiveLength,
        planned_end: Diameter,
    ) -> Self {
        Self {
            index,
            start,
            planned_delta,
            planned_end,
            measurement: None,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn start(&self) -> Diameter {
        self.start
    }

    pub fn planned_delta(&self) -> PositiveLength {
        self.planned_delta
    }

    pub fn planned_end(&self) -> Diameter {
        self.planned_end
    }

    pub fn measurement(&self) -> Option<Diameter> {
        self.measurement
    }

    pub fn set_measurement(&mut self, diameter: Diameter) {
        self.measurement = Some(diameter);
    }

    pub fn clear_measurement(&mut self) {
        self.measurement = None;
    }
}
