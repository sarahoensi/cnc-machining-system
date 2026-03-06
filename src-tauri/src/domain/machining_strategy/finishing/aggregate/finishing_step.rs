// domain/machining_strategy/finishing/finishing_step.rs

use crate::domain::units::{Diameter, PositiveLength};

/// Represents a single machining pass within a finishing execution.
///
/// A step describes the planned machining result for one cut as well as the
/// optional real-world measurement recorded after machining.
///
/// # Concepts
///
/// - `start` — Diameter before the step is executed
/// - `planned_delta` — Intended diameter change (always positive magnitude)
/// - `planned_end` — Expected diameter after applying the step
/// - `measurement` — Optional operator-recorded diameter after machining
///
/// # Domain Invariants
///
/// - `planned_delta` is always positive
/// - `planned_end` is derived from `start` and finishing direction
/// - `measurement`, when present, represents actual machining result
///
/// Measurements may trigger recalculation of subsequent steps at the
/// aggregate level.
#[derive(Debug, Copy, Clone)]
pub struct FinishingStep {
    index: u32,
    start: Diameter,

    /// Planned delta magnitude (positive)
    planned_delta: PositiveLength,

    /// Planned end diameter (start +/- planned_delta)
    planned_end: Diameter,

    /// User-registered measurement for this step (optional)
    measurement: Option<Diameter>,
}

impl FinishingStep {
    /// Creates a new finishing step.
    ///
    /// The step is initially created without a measurement.
    ///
    /// The caller is responsible for ensuring consistency between
    /// `start`, `planned_delta`, `planned_end`, and finishing direction.
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

    /// Step number within execution sequence (1-based).
    pub fn index(&self) -> u32 { self.index }

    /// Planned diameter before this machining step.
    pub fn start(&self) -> Diameter { self.start }

    /// Planned diameter change magnitude for this step.
    pub fn planned_delta(&self) -> PositiveLength { self.planned_delta }

    /// Planned diameter after this machining step.
    pub fn planned_end(&self) -> Diameter { self.planned_end }

    /// Returns recorded measurement, if operator has registered one.
    pub fn measurement(&self) -> Option<Diameter> { self.measurement }

    /// Registers a measurement for this step.
    ///
    /// This represents the real measured diameter after machining.
    pub fn set_measurement(&mut self, d: Diameter) { self.measurement = Some(d); }

    /// Removes any previously registered measurement.
    ///
    /// Typically used when editing operator input.
    pub fn clear_measurement(&mut self) { self.measurement = None; }
}
