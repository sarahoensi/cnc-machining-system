// domain/test_utils/approx.rs

pub const DEFAULT_EPS: f64 = 1e-9;

pub fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    if !a.is_finite() || !b.is_finite() {
        return false;
    }

    let scale = a.abs().max(b.abs()).max(1.0);
    (a - b).abs() <= eps * scale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_values() {
        assert!(approx_eq(1.0, 1.0, DEFAULT_EPS));
    }

    #[test]
    fn near_values() {
        assert!(approx_eq(1.0, 1.0 + 1e-10, DEFAULT_EPS));
    }

    #[test]
    fn far_values() {
        assert!(!approx_eq(1.0, 1.1, DEFAULT_EPS));
    }

    #[test]
    fn handles_zero_scale() {
        assert!(approx_eq(0.0, 1e-10, DEFAULT_EPS));
    }

    #[test]
    fn rejects_nan() {
        assert!(!approx_eq(f64::NAN, 1.0, DEFAULT_EPS));
    }

    #[test]
    fn rejects_infinity() {
        assert!(!approx_eq(f64::INFINITY, 1.0, DEFAULT_EPS));
    }
}