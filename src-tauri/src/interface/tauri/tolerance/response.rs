use serde::Serialize;

#[derive(Serialize)]
pub struct ToleranceResponse {
    pub code: String,
    pub zone: String,
    pub grade: i32,
    pub upper_um: f64,
    pub lower_um: f64,
    pub min_mm: f64,
    pub max_mm: f64,
    pub source_table: Option<String>,
    pub source_file: Option<String>,
}

#[derive(Serialize)]
pub struct ToleranceOptionResponse {
    pub feature: String,
    pub zone: String,
    pub grades: Vec<i32>,
}

#[derive(Serialize)]
pub struct ToleranceOptionsResponse {
    pub holes: Vec<ToleranceOptionResponse>,
    pub shafts: Vec<ToleranceOptionResponse>,
}

#[derive(Serialize)]
pub struct FitSummaryResponse {
    pub min_clearance_mm: f64,
    pub max_clearance_mm: f64,
    #[serde(rename = "type")]
    pub fit_type: String,
}

#[derive(Serialize)]
pub struct FitResponse {
    pub nominal_mm: f64,
    pub hole: ToleranceResponse,
    pub shaft: ToleranceResponse,
    pub fit: FitSummaryResponse,
}
