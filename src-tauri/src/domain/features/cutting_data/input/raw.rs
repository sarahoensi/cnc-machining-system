// raw_input.rs

#[derive(Debug, Clone)]
pub struct RawCuttingInput {
    pub d: Option<f64>,
    pub vc: Option<f64>,
    pub n: Option<f64>,
    pub f: Option<f64>,
    pub fz: Option<f64>,
    pub z: Option<u32>,
}
