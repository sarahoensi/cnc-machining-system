// domain/machining_strategy/finishing_execution.rs

use crate::domain::machining_strategy::strategy_error::StrategyError;
use crate::domain::{Diameter, Length};

use super::{FinishingPlan, FinishingStep};

const EPS: f64 = 1e-12;
const END_TOL: f64 = 1e-9;

#[derive(Debug, Clone)]
pub struct FinishingExecution {
    plan: FinishingPlan,
    steps: Vec<FinishingStep>,
}

impl FinishingExecution {
    pub fn new(plan: FinishingPlan) -> Result<Self, StrategyError> {
        let steps = build_steps_from_start(
            plan.start(),
            plan.target(),
            plan.cuts(),
            plan.expected_step(),
            plan.direction_sign(),
        )?;

        Ok(Self { plan, steps })
    }

    pub fn plan(&self) -> FinishingPlan { self.plan }
    pub fn steps(&self) -> &[FinishingStep] { &self.steps }

    /// Register (or update) a measurement for a given 1-based step number.
    /// This will recalculate all remaining steps after this one.
    ///
    /// Workflow rule:
    /// - If a later step already has a measurement, earlier steps are locked.
    ///   (You may still edit the *last measured* step.)
    pub fn register_measurement(
        &mut self,
        step_number: u32,
        measured: Diameter,
    ) -> Result<(), StrategyError> {
        let idx = to_index(step_number, self.steps.len())?;

        // Lock earlier steps if we already have later measurements
        self.ensure_step_is_editable(idx)?;

        // Validate measurement direction: it must not pass the target in the wrong way
        self.validate_measurement_against_target(measured)?;

        // Store measurement
        self.steps[idx].set_measurement(measured);

        // Recalculate the rest based on this measurement as the new "start"
        self.recalculate_from(idx + 1, measured)?;

        Ok(())
    }

    /// Remove a measurement and rebuild everything after that step using the plan's original expected step.
    /// (Optional helper if you support "reset this row" in UI later.)
    ///
    /// Workflow rule:
    /// - You cannot clear a measurement on an earlier step if there exists a later measurement.
    pub fn clear_measurement(&mut self, step_number: u32) -> Result<(), StrategyError> {
        let idx = to_index(step_number, self.steps.len())?;

        // Lock earlier steps if we already have later measurements
        self.ensure_step_is_editable(idx)?;

        // Clear this step’s measurement
        self.steps[idx].clear_measurement();

        // Determine new base start:
        // - if previous step has a measurement, use it
        // - else use its planned_end
        // - else use plan start (for idx == 0)
        let base = if idx == 0 {
            self.plan.start()
        } else {
            self.steps[idx - 1]
                .measurement()
                .unwrap_or(self.steps[idx - 1].planned_end())
        };

        // Rebuild remaining steps using the ORIGINAL expected_step (uniform)
        self.recalculate_uniform_from(idx, base)?;

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Workflow / locking
    // ---------------------------------------------------------------------

    /// Returns the last step index (0-based) that has a measurement, if any.
    fn last_measured_index(&self) -> Option<usize> {
        self.steps
            .iter()
            .rposition(|s| s.measurement().is_some())
    }

    /// Enforces: if there is a later measured step, earlier steps cannot be modified.
    /// Editing the last measured step itself is allowed.
    fn ensure_step_is_editable(&self, idx: usize) -> Result<(), StrategyError> {
        if let Some(last_idx) = self.last_measured_index() {
            if idx < last_idx {
                return Err(StrategyError::InvalidInputs(
                    "cannot modify an earlier step after a later step has been measured",
                ));
            }
        }
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Internal validation & recalculation logic
    // ---------------------------------------------------------------------

    fn validate_measurement_against_target(&self, measured: Diameter) -> Result<(), StrategyError> {
        let target = self.plan.target().mm_value();
        let m = measured.mm_value();
        let dir = self.plan.direction_sign();

        // Inner: increasing; should not be > target (allow small eps)
        // Outer: decreasing; should not be < target (allow small eps)
        if dir > 0.0 && m > target + EPS {
            return Err(StrategyError::InvalidInputs(
                "measurement exceeds target (inner mode)",
            ));
        }
        if dir < 0.0 && m < target - EPS {
            return Err(StrategyError::InvalidInputs(
                "measurement exceeds target (outer mode)",
            ));
        }
        Ok(())
    }

    fn recalculate_from(
        &mut self,
        start_index: usize,
        last_measured: Diameter,
    ) -> Result<(), StrategyError> {
        if start_index >= self.steps.len() {
            return Ok(()); // nothing left
        }

        let remaining_steps = self.steps.len() - start_index;
        if remaining_steps == 0 {
            return Ok(());
        }

        let target = self.plan.target().mm_value();
        let current = last_measured.mm_value();
        let dir = self.plan.direction_sign();

        let remaining_delta_mag = (target - current).abs();

        // If we’re effectively at target, remaining delta is ~0; but we still have steps.
        if remaining_delta_mag <= EPS {
            return Err(StrategyError::ImpossiblePlan(
                "no remaining delta but still remaining steps",
            ));
        }

        let new_step_mag = remaining_delta_mag / remaining_steps as f64;
        let new_step = Length::mm_positive(new_step_mag)
            .map_err(|_| StrategyError::InvalidInputs("computed step was not > 0"))?;

        // Rebuild steps from start_index forward, clearing measurements in remaining steps
        let mut start_d = last_measured;

        for i in start_index..self.steps.len() {
            let step_no = (i + 1) as u32;

            let end_val = start_d.mm_value() + dir * new_step.mm_value();
            let end_d = Diameter::mm(end_val)
                .map_err(|_| StrategyError::ImpossiblePlan("computed diameter invalid"))?;

            self.steps[i] = FinishingStep::new(step_no, start_d, new_step, end_d);
            start_d = end_d;
        }

        // Ensure last planned end equals target (within tolerance)
        let last_end = self
            .steps
            .last()
            .ok_or(StrategyError::ImpossiblePlan("no steps"))?
            .planned_end()
            .mm_value();

        if (last_end - target).abs() > END_TOL {
            return Err(StrategyError::ImpossiblePlan(
                "recalculation did not reach target",
            ));
        }

        Ok(())
    }

    fn recalculate_uniform_from(
        &mut self,
        start_index: usize,
        base: Diameter,
    ) -> Result<(), StrategyError> {
        // Rebuild remaining steps using the ORIGINAL expected_step, not adaptive
        if start_index >= self.steps.len() {
            return Ok(());
        }

        let dir = self.plan.direction_sign();
        let step = self.plan.expected_step();
        let mut start_d = base;

        for i in start_index..self.steps.len() {
            let step_no = (i + 1) as u32;

            let end_val = start_d.mm_value() + dir * step.mm_value();
            let end_d = Diameter::mm(end_val)
                .map_err(|_| StrategyError::ImpossiblePlan("computed diameter invalid"))?;

            let mut s = FinishingStep::new(step_no, start_d, step, end_d);

            // Reset measurement for rebuilt steps (typical "reset remainder" behavior)
            s.clear_measurement();

            self.steps[i] = s;
            start_d = end_d;
        }

        Ok(())
    }
}

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

fn to_index(step_number: u32, len: usize) -> Result<usize, StrategyError> {
    if step_number == 0 {
        return Err(StrategyError::InvalidInputs("step_number must be 1-based"));
    }
    let idx = (step_number - 1) as usize;
    if idx >= len {
        return Err(StrategyError::InvalidInputs("step_number out of range"));
    }
    Ok(idx)
}

fn build_steps_from_start(
    start: Diameter,
    target: Diameter,
    cuts: u32,
    step: Length,
    dir: f64,
) -> Result<Vec<FinishingStep>, StrategyError> {
    let mut steps = Vec::with_capacity(cuts as usize);
    let mut current = start;

    for i in 0..cuts {
        let index = i + 1;

        let end_val = current.mm_value() + dir * step.mm_value();
        let end = Diameter::mm(end_val)
            .map_err(|_| StrategyError::ImpossiblePlan("computed diameter invalid"))?;

        steps.push(FinishingStep::new(index, current, step, end));
        current = end;
    }

    // ensure last end ~= target
    let last_end = steps
        .last()
        .ok_or(StrategyError::ImpossiblePlan("no steps"))?
        .planned_end()
        .mm_value();

    if (last_end - target.mm_value()).abs() > END_TOL {
        return Err(StrategyError::ImpossiblePlan(
            "initial plan does not end exactly at target (check rounding/units)",
        ));
    }

    Ok(steps)
}
