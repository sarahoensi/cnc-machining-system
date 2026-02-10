// interface/tauri/helix/request.rs

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SolveHelixRequest {

    Pitch {
        mode: HelixMode,
        diameter_mm: f64,
        tool_diameter_mm: f64,
        pitch_mm_per_rev: f64,
    },

    Angle {
        mode: HelixMode,
        diameter_mm: f64,
        tool_diameter_mm: f64,
        angle_deg: f64,
    },
}

#[derive(Deserialize)]
pub enum HelixMode {
    Inner,
    Outer,
}
