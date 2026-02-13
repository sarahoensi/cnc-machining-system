
//! Output DTO for a single finishing execution step.
//!
//! This module provides a transport type used by external interfaces to render
//! planned and measured values for one finishing step.


/// Output DTO representing one step in a finishing execution lifecycle.
///
/// This is an application output contract returned by finishing use cases.
/// It includes planned values and optional measured diameter feedback.
///
/// Unit expectations:
/// - All length values are expressed in millimeters (`mm`).
pub struct FinishingStepOutput {
    /// Step index in execution order.
    pub index: u32,
    /// Diameter at the start of the step (`mm`).
    pub start_mm: f64,
    /// Planned diameter delta to remove during the step (`mm`).
    pub planned_delta_mm: f64,
    /// Planned resulting diameter after this step (`mm`).
    pub planned_end_mm: f64,
    /// Recorded measured diameter after the step, when available (`mm`).
    pub measurement_mm: Option<f64>,
}

