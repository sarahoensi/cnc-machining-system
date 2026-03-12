// domain/machining/finishing/mod.rs

mod finishing_mode;
pub mod error;

pub mod planning;
pub mod execution;

pub use finishing_mode::FinishingMode;
pub use error::FinishingError;

pub use planning::{
    FinishingPlan,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
};

pub use execution::{
    FinishingExecution,
    FinishingStep,
};
