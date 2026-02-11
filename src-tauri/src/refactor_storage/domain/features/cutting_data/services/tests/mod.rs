// cutting_data/services/tests/mod.rs

pub(super) fn approx(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "Expected ~{b}, got {a} (tol {tol})"
    );
}

mod unit;
