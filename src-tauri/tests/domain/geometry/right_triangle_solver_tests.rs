// tests/domain/geometry/right_triangle_solver_tests.rs

use cnc_machining_system_lib::domain::{
    RightTriangle, RightTriangleSolver, units::{AcuteAngle,  PositiveLength}
};
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

const TRI_345_ALPHA: f64 = 36.869897645844;

// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------

fn len(v: f64) -> PositiveLength {
    PositiveLength::mm(v).unwrap()
}


fn angle_deg(v: f64) -> AcuteAngle {
    AcuteAngle::degrees(v).unwrap()
}

fn t_345() -> RightTriangle {
    RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap()
}

fn assert_triangle_close(t1: &RightTriangle, t2: &RightTriangle) {
    assert!(approx_eq(t1.a().mm_value(), t2.a().mm_value(), DEFAULT_EPS));
    assert!(approx_eq(t1.b().mm_value(), t2.b().mm_value(), DEFAULT_EPS));
    assert!(approx_eq(t1.c().mm_value(), t2.c().mm_value(), DEFAULT_EPS));

    assert!(approx_eq(
        t1.alpha().degrees_value(),
        t2.alpha().degrees_value(),
        DEFAULT_EPS
    ));

    assert!(approx_eq(
        t1.beta().degrees_value(),
        t2.beta().degrees_value(),
        DEFAULT_EPS
    ));
}

fn assert_output_invariants(t: &RightTriangle) {
    let lhs = t.a().mm_value().powi(2) + t.b().mm_value().powi(2);
    let rhs = t.c().mm_value().powi(2);

    assert!(approx_eq(lhs, rhs, DEFAULT_EPS));

    let sum = t.alpha().degrees_value() + t.beta().degrees_value();
    assert!(approx_eq(sum, 90.0, DEFAULT_EPS));

    assert!(t.a().mm_value() > 0.0);
    assert!(t.b().mm_value() > 0.0);
    assert!(t.c().mm_value() > 0.0);
}

//
// =========================================================
// HAPPY PATH TESTS
// =========================================================
//

#[test]
fn constructors_build_expected_345_triangle() {
    let tests = [
        RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap(),
        RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0)).unwrap(),
        RightTriangleSolver::from_other_leg_and_hypotenuse(len(4.0), len(5.0)).unwrap(),
        RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(TRI_345_ALPHA)).unwrap(),
        RightTriangleSolver::from_leg_and_opposite_angle(len(3.0), angle_deg(TRI_345_ALPHA)).unwrap(),
        RightTriangleSolver::from_adjacent_leg_and_angle(len(4.0), angle_deg(TRI_345_ALPHA)).unwrap(),
    ];

    for t in tests {
        assert_output_invariants(&t);
        assert!(approx_eq(t.c().mm_value(), 5.0, DEFAULT_EPS));
    }
}

//
// =========================================================
// CROSS PATH CONSISTENCY
// =========================================================
//

#[test]
fn all_constructor_paths_produce_identical_triangle() {
    let base = t_345();

    let variants = [
        RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0)).unwrap(),
        RightTriangleSolver::from_other_leg_and_hypotenuse(len(4.0), len(5.0)).unwrap(),
        RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(TRI_345_ALPHA)).unwrap(),
        RightTriangleSolver::from_leg_and_opposite_angle(len(3.0), angle_deg(TRI_345_ALPHA)).unwrap(),
        RightTriangleSolver::from_adjacent_leg_and_angle(len(4.0), angle_deg(TRI_345_ALPHA)).unwrap(),
    ];

    for t in variants {
        assert_triangle_close(&base, &t);
    }
}

//
// =========================================================
// ROUND TRIP TESTS
// =========================================================
//

#[test]
fn round_trip_reconstruction() {
    let t = RightTriangleSolver::from_legs(len(7.3), len(2.9)).unwrap();

    let variants = [
        RightTriangleSolver::from_hypotenuse_and_angle(t.c(), t.alpha()).unwrap(),
        RightTriangleSolver::from_leg_and_opposite_angle(t.a(), t.alpha()).unwrap(),
        RightTriangleSolver::from_adjacent_leg_and_angle(t.b(), t.alpha()).unwrap(),
    ];

    for reconstructed in variants {
        assert_triangle_close(&t, &reconstructed);
    }
}

//
// =========================================================
// SCALING + SYMMETRY
// =========================================================
//

#[test]
fn scaling_preserves_angles() {
    let t1 = RightTriangleSolver::from_legs(len(3.2), len(4.7)).unwrap();
    let t2 = RightTriangleSolver::from_legs(len(32.0), len(47.0)).unwrap();

    assert!(approx_eq(
        t1.alpha().degrees_value(),
        t2.alpha().degrees_value(),
        DEFAULT_EPS
    ));
}

#[test]
fn swapping_legs_swaps_angles() {
    let t1 = RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap();
    let t2 = RightTriangleSolver::from_legs(len(4.0), len(3.0)).unwrap();

    assert!(approx_eq(t1.alpha().degrees_value(), t2.beta().degrees_value(), DEFAULT_EPS));
}

//
// =========================================================
// PROPERTY STYLE RANDOMIZED TEST
// =========================================================
//

#[test]
fn random_legs_always_produce_valid_triangle() {
    for i in 1..100 {
        let a = i as f64 * 0.73 + 0.1;
        let b = i as f64 * 1.11 + 0.3;

        let t = RightTriangleSolver::from_legs(len(a), len(b)).unwrap();
        assert_output_invariants(&t);
    }
}

//
// =========================================================
// NUMERICAL EDGE CASES
// =========================================================
//

#[test]
fn tolerates_small_rounding_noise() {
    let r = RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0000000000000001));
    assert!(r.is_ok());
}

#[test]
fn extremely_small_opposite_angle_rejected() {
    let r = RightTriangleSolver::from_leg_and_opposite_angle(len(3.0), angle_deg(1e-11));
    assert!(r.is_err());
}

#[test]
fn near_ninety_angle_still_valid() {
    let t = RightTriangleSolver::from_leg_and_opposite_angle(len(10.0), angle_deg(89.999999)).unwrap();
    assert_output_invariants(&t);
}

//
// =========================================================
// ERROR CASES
// =========================================================
//

#[test]
fn rejects_invalid_lengths() {
    assert!(PositiveLength::mm(0.0).is_err());
    assert!(PositiveLength::mm(-3.0).is_err());
}

#[test]
fn rejects_invalid_angles() {
    for a in [0.0, 90.0, -30.0, 120.0] {
        assert!(AcuteAngle::degrees(a).is_err());
    }
}
#[test]
fn rejects_impossible_geometry() {
    assert!(
       ( RightTriangleSolver::from_leg_and_hypotenuse(len(5.0), len(3.0)).is_err()
    ));
}


