// domain/machining/finishing/error.rs

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum FinishingError {
    #[error("Start diameter ({start_mm}) and target diameter ({target_mm}) are incompatible with the selected direction")]
    InvalidModeDirection { start_mm: f64, target_mm: f64 },

    #[error("Start and target diameters must differ")]
    DiametersMustDiffer,

    #[error("Cut count must be greater than zero (got {cuts})")]
    InvalidCutCount { cuts: u32 },

    #[error("Radial engagement must be positive (got {value_mm} mm)")]
    InvalidRadialEngagement { value_mm: f64 },

    #[error("Computed step must be positive (got {value_mm} mm)")]
    ComputedStepNotPositive { value_mm: f64 },

    #[error("Impossible machining plan: {reason}")]
    ImpossiblePlan { reason: &'static str },

    #[error("Step number must start at 1")]
    StepNumberMustBeOneBased,

    #[error("Step number {step_number} is out of range (total steps: {total_steps})")]
    StepNumberOutOfRange {
        step_number: u32,
        total_steps: usize,
    },

    #[error("Step {attempted_step} is locked (last measured step: {last_measured_step})")]
    StepLocked {
        attempted_step: u32,
        last_measured_step: u32,
    },

    #[error("Measurement {measured_mm} mm is outside bounds ({start_mm} mm → {target_mm} mm)")]
    MeasurementOutOfBounds {
        measured_mm: f64,
        start_mm: f64,
        target_mm: f64,
    },

    #[error("Measurement moved backwards ({previous_mm} mm → {measured_mm} mm)")]
    MeasurementBackwards { previous_mm: f64, measured_mm: f64 },

    #[error("Measurement {measured_mm} mm exceeds target {target_mm} mm")]
    MeasurementExceedsTarget { measured_mm: f64, target_mm: f64 },
}
