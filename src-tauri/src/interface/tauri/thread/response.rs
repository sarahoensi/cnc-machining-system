use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadPitchOptionResponse {
    pub value: String,
    pub label: String,
    pub pitch_mm: f64,
    pub series: String,
    pub is_default_pitch: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadSizeOptionResponse {
    pub value: String,
    pub label: String,
    pub major_diameter_mm: f64,
    pub pitches: Vec<ThreadPitchOptionResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadTypeOptionResponse {
    pub value: String,
    pub label: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadOptionsResponse {
    pub types: Vec<ThreadTypeOptionResponse>,
    pub metric: Vec<ThreadSizeOptionResponse>,
    pub unc: Vec<ThreadSizeOptionResponse>,
    pub unf: Vec<ThreadSizeOptionResponse>,
    pub bsp: Vec<ThreadSizeOptionResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SolveThreadResponse {
    pub drill_diameter_mm: f64,
    pub thread_depth_mm: f64,
}
