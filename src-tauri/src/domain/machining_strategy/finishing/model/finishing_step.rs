// domain/machining_strategy/finishing/finishing_step.rs

use crate::domain::{Diameter, Length};

/// One step in the finishing execution table.
#[derive(Debug, Copy, Clone)]
pub struct FinishingStep {
    index: u32,
    start: Diameter,

    /// Planned delta magnitude (positive)
    planned_delta: Length,

    /// Planned end diameter (start +/- planned_delta)
    planned_end: Diameter,

    /// User-registered measurement for this step (optional)
    measurement: Option<Diameter>,
}

impl FinishingStep {
    pub fn new(
        index: u32,
        start: Diameter,
        planned_delta: Length,
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

    pub fn index(&self) -> u32 { self.index }
    pub fn start(&self) -> Diameter { self.start }
    pub fn planned_delta(&self) -> Length { self.planned_delta }
    pub fn planned_end(&self) -> Diameter { self.planned_end }

    pub fn measurement(&self) -> Option<Diameter> { self.measurement }
    pub fn set_measurement(&mut self, d: Diameter) { self.measurement = Some(d); }
    pub fn clear_measurement(&mut self) { self.measurement = None; }
}
