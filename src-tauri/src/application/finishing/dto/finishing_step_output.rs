

pub struct FinishingStepOutput {
    pub index: u32,
    pub start_mm: f64,
    pub planned_delta_mm: f64,
    pub planned_end_mm: f64,
    pub measurement_mm: Option<f64>,
}

