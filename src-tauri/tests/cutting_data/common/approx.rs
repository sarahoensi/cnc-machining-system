// tests/cutting_data/common/approx.rs

pub fn approx_eq(a: f64, b: f64, rel: f64) -> bool {
    let diff = (a - b).abs();
    diff <= rel * a.abs().max(b.abs()).max(1.0)
}
