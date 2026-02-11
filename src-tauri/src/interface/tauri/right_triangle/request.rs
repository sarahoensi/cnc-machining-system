// interface/tauri/right_triangle/request.rs

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SolveRightTriangleRequest {
    Legs {
        a_mm: f64,
        b_mm: f64,
    },
    LegAndHypotenuse {
        a_mm: f64,
        c_mm: f64,
    },
    OtherLegAndHypotenuse {
        b_mm: f64,
        c_mm: f64,
    },
    HypotenuseAndAngle {
        c_mm: f64,
        alpha_deg: f64,
    },
}
