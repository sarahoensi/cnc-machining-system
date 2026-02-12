// interface/tauri/finishing/mod.rs
mod request;
mod response;
mod mapping;
mod command;

// ---------- Commands ----------
pub use command::{
    generate_finishing_plan,
    register_finishing_measurement,
};

// ---------- DTO ----------
pub use request::{
    GenerateFinishingPlanRequest,
    RegisterFinishingMeasurementRequest,
    FinishingMode,
};

pub use response::{
    FinishingExecutionResponse,
    FinishingStepResponse,
};
