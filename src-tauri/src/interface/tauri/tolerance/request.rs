use serde::Deserialize;

#[derive(Deserialize)]
pub struct LookupIso286ToleranceRequest {
    pub feature: String,
    pub nominal_mm: f64,
    pub code: String,
}

#[derive(Deserialize)]
pub struct CalculateIso286FitRequest {
    pub nominal_mm: f64,
    pub hole: String,
    pub shaft: String,
}
