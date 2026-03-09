// domain/machining_strategy/finishing_execution.rs

use crate::domain::{
    FinishingMode, FinishingPlan, FinishingPlanner, FinishingPlanning, FinishingRequest, FinishingStep, machining_strategy::strategy_error::StrategyError, units::{Diameter, PositiveLength}
};

const EPS: f64 = 1e-12;
const END_TOL: f64 = 1e-9;
use super::FinishingExecutionId;

/// Represents a running finishing operation.
///
/// A `FinishingExecution` combines:
///
/// - A static [`FinishingPlan`] describing intended machining
/// - A dynamic list of [`FinishingStep`] representing runtime progress
///
/// # Responsibilities
///
/// This aggregate:
///
/// - Tracks operator measurements
/// - Recalculates remaining steps when measurements deviate from plan
/// - Enforces workflow rules and machining direction constraints
///
/// # Workflow Rules
///
/// - Steps are executed sequentially
/// - Measurements lock earlier steps once later steps are recorded
/// - Remaining steps are recalculated after each measurement
///
/// # Domain Guarantees
///
/// - Planned steps always reach the target diameter
/// - Measurements cannot pass target in wrong direction
/// - Step numbering is always 1-based externally
#[derive(Debug, Clone)]

pub struct FinishingExecution {
    id: FinishingExecutionId,
    plan: FinishingPlan,
    steps: Vec<FinishingStep>,
}

impl FinishingExecution {
    /// Creates a new finishing execution from a static plan.
    ///
    /// Generates the initial list of machining steps.
    pub fn new(id: FinishingExecutionId, plan: FinishingPlan) -> Result<Self, StrategyError> {
        let steps = build_steps_from_start(
            plan.start(),
            plan.target(),
            plan.cuts(),
            plan.expected_step(),
            plan.mode(),
        )?;

        Ok(Self { id, plan, steps })
    }

    /// Unique identifier of this execution instance.
    pub fn id(&self) -> FinishingExecutionId {
        self.id
    }
    /// Static plan associated with this execution.
    pub fn plan(&self) -> FinishingPlan {
        self.plan
    }

    /// All finishing steps, including planned and measured data.
    pub fn steps(&self) -> &[FinishingStep] {
        &self.steps
    }

    /// Registers or updates a measurement for a given step.
    ///
    /// # Behavior
    ///
    /// - Step numbers are 1-based
    /// - Stores operator measurement
    /// - Recalculates all remaining steps using the measurement as new start
    ///
    /// # Workflow Locking
    ///
    /// If a later step already has a measurement:
    ///
    /// - Earlier steps cannot be edited
    /// - The last measured step may still be modified
    pub fn register_measurement(
        &mut self,
        step_number: u32,
        measured: Diameter,
    ) -> Result<(), StrategyError> {
        let idx = to_index(step_number, self.steps.len())?;

        // Lock earlier steps if we already have later measurements
        self.ensure_step_is_editable(idx)?;

        self.validate_measurement_within_plan_bounds(measured)?;
        self.validate_measurement_progression(idx, measured)?;

        // Validate measurement direction: it must not pass the target in the wrong way
        self.validate_measurement_against_target(measured)?;

        // Store measurement
        self.steps[idx].set_measurement(measured);

        // Recalculate the rest based on this measurement as the new "start"
        self.recalculate_from(idx + 1, measured)?;

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Workflow / locking
    // ---------------------------------------------------------------------

    /// Returns index of last step with a registered measurement.
    fn last_measured_index(&self) -> Option<usize> {
        self.steps.iter().rposition(|s| s.measurement().is_some())
    }

    /// Ensures workflow editing rules are respected.
    ///
    /// Earlier steps become locked once a later step has a measurement.
    fn ensure_step_is_editable(&self, idx: usize) -> Result<(), StrategyError> {
        if let Some(last_idx) = self.last_measured_index() {
            if idx < last_idx {
                return Err(StrategyError::StepLocked {
                    attempted_step: (idx + 1) as u32,
                    last_measured_step: (last_idx + 1) as u32,
                });
            }
        }
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Internal validation & recalculation logic
    // ---------------------------------------------------------------------

    /// Validates measurement does not overshoot target diameter.
    fn validate_measurement_against_target(&self, measured: Diameter) -> Result<(), StrategyError> {
        let target = self.plan.target().mm_value();
        let m = measured.mm_value();
        let mode = self.plan.mode();

        if mode.passes_target(target, m, EPS) {
            return Err(StrategyError::MeasurementExceedsTarget {
                measured_mm: m,
                target_mm: target,
            });
        }
        Ok(())
    }

    fn validate_measurement_within_plan_bounds(
        &self,
        measured: Diameter,
    ) -> Result<(), StrategyError> {
        let start = self.plan.start().mm_value();
        let target = self.plan.target().mm_value();
        let m = measured.mm_value();
        let mode = self.plan.mode();

        if !mode.within_bounds(start, target, m, EPS) {
            return Err(StrategyError::MeasurementOutOfBounds {
                measured_mm: m,
                start_mm: start,
                target_mm: target,
            });
        }

        Ok(())
    }

    fn validate_measurement_progression(
        &self,
        idx: usize,
        measured: Diameter,
    ) -> Result<(), StrategyError> {
        let last_measured = self
            .steps
            .iter()
            .take(idx)
            .rev()
            .find_map(|s| s.measurement());

        let Some(prev) = last_measured else {
            return Ok(());
        };

        let prev_val = prev.mm_value();
        let m = measured.mm_value();
        let mode = self.plan.mode();

        if !mode.progresses_forward(prev_val, m, EPS) {
            return Err(StrategyError::MeasurementBackwards {
                previous_mm: prev_val,
                measured_mm: m,
            });
        }

        Ok(())
    }

    /// Recalculates all steps following a measurement.
    ///
    /// Remaining steps are redistributed evenly to ensure
    /// final planned diameter reaches the target.
    fn recalculate_from(
    &mut self,
    start_index: usize,
    last_measured: Diameter,
) -> Result<(), StrategyError> {

    let remaining_steps = self.steps.len() - start_index;

    if remaining_steps == 0 {
        return Ok(());
    }

    let request = FinishingRequest {
        mode: self.plan.mode(),
        start_diameter: last_measured,
        target_diameter: self.plan.target(),
        planning: FinishingPlanning::ByCuts(remaining_steps as u32),
    };

    let new_plan = FinishingPlanner::generate_plan(request)?;

    // Generate the remaining steps using the same logic as initial build
    let new_steps = build_steps_from_start(
        last_measured,
        self.plan.target(),
        remaining_steps as u32,
        new_plan.expected_step(),
        self.plan.mode(),
    )?;

    // Replace remaining steps
    for (offset, step) in new_steps.into_iter().enumerate() {
        self.steps[start_index + offset] = step;
    }

    Ok(())
}
}

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

fn to_index(step_number: u32, len: usize) -> Result<usize, StrategyError> {
    if step_number == 0 {
        return Err(StrategyError::StepNumberMustBeOneBased);
    }

    let idx = (step_number - 1) as usize;

    if idx >= len {
        return Err(StrategyError::StepNumberOutOfRange {
            step_number,
            total_steps: len,
        });
    }

    Ok(idx)
}

fn build_steps_from_start(
    start: Diameter,
    target: Diameter,
    cuts: u32,
    step: PositiveLength,
    mode: FinishingMode,
) -> Result<Vec<FinishingStep>, StrategyError> {
    if cuts == 0 {
        return Err(StrategyError::ImpossiblePlan {
            reason: "plan contains zero cuts",
        });
    }

    let mut steps = Vec::with_capacity(cuts as usize);
    let mut current = start;

    for i in 0..cuts {
        let index = i + 1;

        let end_val = mode.apply_delta(current.mm_value(), step.mm_value());

        let end = Diameter::mm(end_val).map_err(|_| StrategyError::ImpossiblePlan {
            reason: "computed diameter invalid",
        })?;

        steps.push(FinishingStep::new(index, current, step, end));
        current = end;
    }

    // Internal invariant: steps cannot be empty here
    let last_end = steps
        .last()
        .expect("FinishingExecution invariant violated: steps cannot be empty")
        .planned_end()
        .mm_value();

    let target_mm = target.mm_value();

    if (last_end - target_mm).abs() > END_TOL {
        return Err(StrategyError::RecalculationDidNotReachTarget {
            final_mm: last_end,
            target_mm,
        });
    }

    Ok(steps)
}
