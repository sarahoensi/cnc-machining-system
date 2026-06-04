// domain/machining/finishing/mod.rs

pub mod error;
mod finishing_mode;

pub mod execution;
pub mod planning;

pub use error::FinishingError;
pub use finishing_mode::FinishingMode;

pub use planning::{FinishingPlan, FinishingPlanner, FinishingPlanning, FinishingRequest};

pub use execution::{FinishingExecution, FinishingStep};
