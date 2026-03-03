// domain/machining_strategy/finishing/finishing_plan.rs

use crate::domain::{
    FinishingMode, units::{Diameter, PositiveLength}
};


/// Represents a static finishing plan.
///
/// Defines the start and target diameters, number of cuts,
/// and the expected diameter change per step.
///
/// The plan itself contains no execution state — it is a
/// deterministic description of the intended finishing strategy.
#[derive(Debug, Copy, Clone)]
pub struct FinishingPlan {
    mode: FinishingMode,
    start: Diameter,
    target: Diameter,
    cuts: u32,
    expected_step: PositiveLength, // always positive magnitude
}

impl FinishingPlan {
    /// Returns the finishing mode (inner or outer).
    pub fn mode(&self) -> FinishingMode { self.mode }

    /// Returns the starting diameter.
    pub fn start(&self) -> Diameter { self.start }

    /// Returns the target diameter.
    pub fn target(&self) -> Diameter { self.target }

    /// Returns the number of finishing cuts.
    pub fn cuts(&self) -> u32 { self.cuts }

    /// Returns the expected diameter change per step.
    ///
    /// This value is always a positive magnitude.
    /// Direction is determined by [`FinishingMode`].
    pub fn expected_step(&self) -> PositiveLength { self.expected_step }

    /// Returns the directional sign of the diameter change.
    ///
    /// Delegates to [`FinishingMode::direction_sign`].
    pub fn direction_sign(&self) -> f64 {
        self.mode.direction_sign()
    }
}

impl FinishingPlan {
    /// Creates a new [`FinishingPlan`].
    ///
    /// Intended for internal use by the finishing planner.
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

}
