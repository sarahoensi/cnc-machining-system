#[derive(Debug, Clone, PartialEq)]
pub struct ThreadPitchOptionOutput {
    pub value: String,
    pub label: String,
    pub pitch_mm: f64,
    pub series: String,
    pub is_default_pitch: bool,
    pub tap_drill_basis: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadSizeOptionOutput {
    pub value: String,
    pub label: String,
    pub major_diameter_mm: f64,
    pub pitches: Vec<ThreadPitchOptionOutput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadTypeOptionOutput {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadOptionsOutput {
    pub types: Vec<ThreadTypeOptionOutput>,
    pub metric: Vec<ThreadSizeOptionOutput>,
    pub unc: Vec<ThreadSizeOptionOutput>,
    pub unf: Vec<ThreadSizeOptionOutput>,
    pub bsp: Vec<ThreadSizeOptionOutput>,
    pub npt: Vec<ThreadSizeOptionOutput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolveThreadInput {
    pub thread_type: String,
    pub size: String,
    pub pitch: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolveThreadOutput {
    pub drill_diameter_mm: f64,
    pub thread_depth_mm: f64,
}
