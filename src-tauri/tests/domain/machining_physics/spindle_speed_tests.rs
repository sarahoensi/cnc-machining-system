// tests/domain/machining_physics/spindle_speed_tests.rs

use cnc_machining_system_lib::domain::*;
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

fn dia(v: f64) -> Diameter {
    Diameter::mm(v).unwrap()
}

fn rpm(v: f64) -> Rpm {
    Rpm::new(v).unwrap()
}

fn vc(v: f64) -> CuttingSpeed {
    CuttingSpeed::meters_per_min(v).unwrap()
}

#[test]
fn rpm_from_cutting_speed_known_example() {
    let r = SpindleSpeedCalculator::rpm_from_cutting_speed(
        vc(100.0),
        dia(10.0),
    )
    .unwrap();

    let expected = (100.0 * 1000.0) / (std::f64::consts::PI * 10.0);

    assert!(approx_eq(r.value(), expected, DEFAULT_EPS));
}

#[test]
fn rpm_vc_roundtrip() {
    let d = dia(12.0);
    let original = rpm(750.0);

    let vc = SpindleSpeedCalculator::cutting_speed_from_rpm(original, d).unwrap();
    let reconstructed = SpindleSpeedCalculator::rpm_from_cutting_speed(vc, d).unwrap();

    assert!(approx_eq(original.value(), reconstructed.value(), DEFAULT_EPS));
}

#[test]
fn scaling_diameter_halves_rpm() {
    let r1 = SpindleSpeedCalculator::rpm_from_cutting_speed(vc(100.0), dia(10.0)).unwrap();
    let r2 = SpindleSpeedCalculator::rpm_from_cutting_speed(vc(100.0), dia(20.0)).unwrap();

    assert!(approx_eq(r2.value(), r1.value() / 2.0, DEFAULT_EPS));
}
