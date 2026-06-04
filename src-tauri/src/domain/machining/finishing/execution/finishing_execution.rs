// domain/machining/finishing/execution/finishing_execution.rs

use crate::domain::{
    machining::finishing::{FinishingError, FinishingPlan, FinishingStep},
    units::{Diameter, PositiveLength},
};

const EPS: f64 = 1e-9;

#[derive(Debug, Clone)]
pub struct FinishingExecution {
    plan: FinishingPlan,
    steps: Vec<FinishingStep>,
}

impl FinishingExecution {
    pub fn new(plan: FinishingPlan) -> Result<Self, FinishingError> {
        let steps = build_initial_steps(plan)?;
        Ok(Self { plan, steps })
    }

    pub fn plan(&self) -> FinishingPlan {
        self.plan
    }

    pub fn steps(&self) -> &[FinishingStep] {
        &self.steps
    }

    pub fn active_step(&self) -> Option<u32> {
        self.steps
            .iter()
            .find(|step| step.measurement().is_none())
            .map(|step| step.index())
    }

    pub fn finished(&self) -> bool {
        self.active_step().is_none()
    }

    pub fn register_measurement(
        &mut self,
        step_number: u32,
        measured: Diameter,
    ) -> Result<(), FinishingError> {
        let idx = self.step_index(step_number)?;

        self.ensure_step_is_editable(idx)?;
        self.validate_measurement(idx, measured)?;

        self.steps[idx].set_measurement(measured);

        self.recalculate_remaining_steps(idx + 1, measured)?;

        Ok(())
    }

    fn step_index(&self, step_number: u32) -> Result<usize, FinishingError> {
        if step_number == 0 {
            return Err(FinishingError::StepNumberMustBeOneBased);
        }

        let idx = (step_number - 1) as usize;

        if idx >= self.steps.len() {
            return Err(FinishingError::StepNumberOutOfRange {
                step_number,
                total_steps: self.steps.len(),
            });
        }

        Ok(idx)
    }

    fn ensure_step_is_editable(&self, idx: usize) -> Result<(), FinishingError> {
        if let Some(last_idx) = self.last_measured_index() {
            if idx < last_idx {
                return Err(FinishingError::StepLocked {
                    attempted_step: (idx + 1) as u32,
                    last_measured_step: (last_idx + 1) as u32,
                });
            }
        }

        Ok(())
    }

    fn last_measured_index(&self) -> Option<usize> {
        self.steps
            .iter()
            .rposition(|step| step.measurement().is_some())
    }

    fn validate_measurement(&self, idx: usize, measured: Diameter) -> Result<(), FinishingError> {
        let m = measured.mm_value();
        let start = self.plan.start().mm_value();
        let target = self.plan.target().mm_value();
        let mode = self.plan.mode();

        if !mode.within_bounds(start, target, m, EPS) {
            return Err(FinishingError::MeasurementOutOfBounds {
                measured_mm: m,
                start_mm: start,
                target_mm: target,
            });
        }

        if mode.passes_target(target, m, EPS) {
            return Err(FinishingError::MeasurementExceedsTarget {
                measured_mm: m,
                target_mm: target,
            });
        }

        if let Some(previous) = self
            .steps
            .iter()
            .take(idx)
            .rev()
            .find_map(|step| step.measurement())
        {
            let prev = previous.mm_value();

            if !mode.progresses_forward(prev, m, EPS) {
                return Err(FinishingError::MeasurementBackwards {
                    previous_mm: prev,
                    measured_mm: m,
                });
            }
        }

        Ok(())
    }

    fn recalculate_remaining_steps(
        &mut self,
        start_index: usize,
        current_start: Diameter,
    ) -> Result<(), FinishingError> {
        let remaining_steps = self.steps.len().saturating_sub(start_index);

        if remaining_steps == 0 {
            return Ok(());
        }

        let target = self.plan.target().mm_value();
        let start = current_start.mm_value();
        let remaining_delta = (target - start).abs();

        let step_size = remaining_delta / remaining_steps as f64;
        let planned_delta =
            PositiveLength::mm(step_size).map_err(|_| FinishingError::ComputedStepNotPositive {
                value_mm: step_size,
            })?;

        let mut current = current_start;

        for offset in 0..remaining_steps {
            let idx = start_index + offset;
            let end_val = self.plan.mode().apply_delta(current.mm_value(), step_size);
            let end = Diameter::mm(end_val).map_err(|_| FinishingError::ImpossiblePlan {
                reason: "computed diameter invalid",
            })?;

            self.steps[idx] = FinishingStep::new((idx + 1) as u32, current, planned_delta, end);

            current = end;
        }

        Ok(())
    }
}

fn build_initial_steps(plan: FinishingPlan) -> Result<Vec<FinishingStep>, FinishingError> {
    let cuts = plan.cuts();

    if cuts == 0 {
        return Err(FinishingError::InvalidCutCount { cuts });
    }

    let mut steps = Vec::with_capacity(cuts as usize);
    let mut current = plan.start();

    for i in 0..cuts {
        let end_val = plan
            .mode()
            .apply_delta(current.mm_value(), plan.expected_step().mm_value());

        let end = Diameter::mm(end_val).map_err(|_| FinishingError::ImpossiblePlan {
            reason: "computed diameter invalid",
        })?;

        steps.push(FinishingStep::new(
            i + 1,
            current,
            plan.expected_step(),
            end,
        ));

        current = end;
    }

    Ok(steps)
}
