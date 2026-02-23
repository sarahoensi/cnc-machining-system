// domain/machining_strategy/strategy_error.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StrategyError {

    // ---------------------------------------------------------
    // Planning errors
    // ---------------------------------------------------------

    InvalidModeDirection {
        start_mm: f64,
        target_mm: f64,
    },

    DiametersMustDiffer,

    InvalidCutCount {
        cuts: u32,
    },

    InvalidRadialEngagement {
        value_mm: f64,
    },

    ComputedStepNotPositive {
        value_mm: f64,
    },

    ImpossiblePlan {
        reason: &'static str,
    },

    // ---------------------------------------------------------
    // Execution workflow errors
    // ---------------------------------------------------------

    StepNumberMustBeOneBased,

    StepNumberOutOfRange {
        step_number: u32,
        total_steps: usize,
    },

    StepLocked {
        attempted_step: u32,
        last_measured_step: u32,
    },

    MeasurementOutOfBounds {
        measured_mm: f64,
        start_mm: f64,
        target_mm: f64,
    },

    MeasurementBackwards {
        previous_mm: f64,
        measured_mm: f64,
    },

    MeasurementExceedsTarget {
        measured_mm: f64,
        target_mm: f64,
    },

    RecalculationDidNotReachTarget {
        final_mm: f64,
        target_mm: f64,
    },

    DivisionByZero,
}


impl fmt::Display for StrategyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StrategyError {}