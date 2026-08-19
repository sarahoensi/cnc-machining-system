#[derive(Debug, Clone, PartialEq)]
pub struct ToleranceResult {
    pub code: String,
    pub zone: String,
    pub grade: i32,
    pub upper_um: f64,
    pub lower_um: f64,
    pub mid_um: f64,
    pub min_mm: f64,
    pub max_mm: f64,
    pub mid_mm: f64,
    pub source_table: Option<String>,
    pub source_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToleranceOption {
    pub feature: String,
    pub zone: String,
    pub grades: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToleranceOptions {
    pub holes: Vec<ToleranceOption>,
    pub shafts: Vec<ToleranceOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FitSummary {
    pub min_clearance_mm: f64,
    pub max_clearance_mm: f64,
    pub fit_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FitResult {
    pub nominal_mm: f64,
    pub hole: ToleranceResult,
    pub shaft: ToleranceResult,
    pub fit: FitSummary,
}
