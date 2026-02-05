// cutting_data/input/raw.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RawCuttingInput {
    pub d: Option<f64>,
    pub vc: Option<f64>,
    pub n: Option<f64>,
    pub f: Option<f64>,
    pub fz: Option<f64>,
    pub z: Option<u32>,
}
