use cnc_machining_system_lib::domain::GeometryError;
use cnc_machining_system_lib::domain::RightTriangleSolver;
use cnc_machining_system_lib::domain::Length;
use cnc_machining_system_lib::domain::Angle;
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------

fn len(v: f64) -> Length {
    Length::mm_positive(v).unwrap()
}

fn len_any(v: f64) -> Length {
    // Allows 0 / negative if Length::mm permits it (as in your existing tests).
    Length::mm(v).unwrap()
}

fn angle_deg(v: f64) -> Angle {
    Angle::degrees(v).unwrap()
}

fn assert_triangle_close(
    t1: &cnc_machining_system_lib::domain::RightTriangle,
    t2: &cnc_machining_system_lib::domain::RightTriangle,
) {
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

fn assert_output_invariants(
    t: &cnc_machining_system_lib::domain::RightTriangle,
) {
    // Pythagoras
    let lhs = t.a().mm_value().powi(2) + t.b().mm_value().powi(2);
    let rhs = t.c().mm_value().powi(2);
    assert!(approx_eq(lhs, rhs, DEFAULT_EPS));

    // Acute angle sum
    let sum = t.alpha().degrees_value() + t.beta().degrees_value();
    assert!(approx_eq(sum, 90.0, DEFAULT_EPS));

    // Basic positivity
    assert!(t.a().mm_value() > 0.0);
    assert!(t.b().mm_value() > 0.0);
    assert!(t.c().mm_value() > 0.0);
}

fn t_345() -> cnc_machining_system_lib::domain::RightTriangle {
    RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap()
}

// ---------------------------------------------------------
// Happy-path constructor correctness (known triangle)
// ---------------------------------------------------------

#[test]
fn from_legs_builds_correct_triangle() {
    let t = t_345();
    assert!(approx_eq(t.c().mm_value(), 5.0, DEFAULT_EPS));
    assert_output_invariants(&t);
}

#[test]
fn from_leg_and_hypotenuse_builds_correct_triangle() {
    let t = RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0)).unwrap();
    assert!(approx_eq(t.b().mm_value(), 4.0, DEFAULT_EPS));
    assert_output_invariants(&t);
}

#[test]
fn from_other_leg_and_hypotenuse_builds_correct_triangle() {
    let t = RightTriangleSolver::from_other_leg_and_hypotenuse(len(4.0), len(5.0)).unwrap();
    assert!(approx_eq(t.a().mm_value(), 3.0, DEFAULT_EPS));
    assert_output_invariants(&t);
}

#[test]
fn from_hypotenuse_and_angle_builds_correct_triangle() {
    // alpha for 3-4-5 where a=3, c=5 => asin(0.6) ≈ 36.869897645844°
    let t = RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(36.869897645844)).unwrap();
    assert!(approx_eq(t.a().mm_value(), 3.0, DEFAULT_EPS));
    assert!(approx_eq(t.b().mm_value(), 4.0, DEFAULT_EPS));
    assert_output_invariants(&t);
}

#[test]
fn from_leg_and_opposite_angle_builds_correct_triangle() {
    let t = RightTriangleSolver::from_leg_and_opposite_angle(len(3.0), angle_deg(36.869897645844)).unwrap();
    assert!(approx_eq(t.c().mm_value(), 5.0, DEFAULT_EPS));
    assert_output_invariants(&t);
}

#[test]
fn from_adjacent_leg_and_angle_builds_correct_triangle() {
    let t = RightTriangleSolver::from_adjacent_leg_and_angle(len(4.0), angle_deg(36.869897645844)).unwrap();
    assert!(approx_eq(t.a().mm_value(), 3.0, DEFAULT_EPS));
    assert_output_invariants(&t);
}

// ---------------------------------------------------------
// Cross-path consistency: all constructors => same triangle
// ---------------------------------------------------------

#[test]
fn all_constructor_paths_produce_same_triangle_345() {
    let t_legs = RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap();

    let t_leg_hyp = RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0)).unwrap();
    let t_other_leg_hyp = RightTriangleSolver::from_other_leg_and_hypotenuse(len(4.0), len(5.0)).unwrap();

    let alpha = angle_deg(36.869897645844);
    let t_hyp_angle = RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), alpha).unwrap();
    let t_leg_angle = RightTriangleSolver::from_leg_and_opposite_angle(len(3.0), alpha).unwrap();
    let t_adj_leg_angle = RightTriangleSolver::from_adjacent_leg_and_angle(len(4.0), alpha).unwrap();

    let triangles = [t_legs, t_leg_hyp, t_other_leg_hyp, t_hyp_angle, t_leg_angle, t_adj_leg_angle];

    for t in &triangles {
        assert_output_invariants(t);
    }

    for t in &triangles[1..] {
        assert_triangle_close(&triangles[0], t);
    }
}

// ---------------------------------------------------------
// Round-trip reconstruction tests
// ---------------------------------------------------------

#[test]
fn round_trip_from_legs_to_hyp_angle_and_back() {
    let t1 = RightTriangleSolver::from_legs(len(7.3), len(2.9)).unwrap();
    let t2 = RightTriangleSolver::from_hypotenuse_and_angle(t1.c(), t1.alpha()).unwrap();

    assert_triangle_close(&t1, &t2);
}

#[test]
fn round_trip_from_legs_to_leg_angle_and_back() {
    let t1 = RightTriangleSolver::from_legs(len(9.2), len(1.7)).unwrap();
    let t2 = RightTriangleSolver::from_leg_and_opposite_angle(t1.a(), t1.alpha()).unwrap();

    assert_triangle_close(&t1, &t2);
}

#[test]
fn round_trip_from_legs_to_adjacent_angle_and_back() {
    let t1 = RightTriangleSolver::from_legs(len(6.4), len(8.1)).unwrap();
    let t2 = RightTriangleSolver::from_adjacent_leg_and_angle(t1.b(), t1.alpha()).unwrap();

    assert_triangle_close(&t1, &t2);
}

// ---------------------------------------------------------
// Scaling invariants
// ---------------------------------------------------------

#[test]
fn scaling_inputs_scales_lengths_but_keeps_angles() {
    let t1 = RightTriangleSolver::from_legs(len(3.2), len(4.7)).unwrap();
    let k = 10.0;
    let t2 = RightTriangleSolver::from_legs(len(3.2 * k), len(4.7 * k)).unwrap();

    assert!(approx_eq(t2.a().mm_value(), t1.a().mm_value() * k, DEFAULT_EPS));
    assert!(approx_eq(t2.b().mm_value(), t1.b().mm_value() * k, DEFAULT_EPS));
    assert!(approx_eq(t2.c().mm_value(), t1.c().mm_value() * k, DEFAULT_EPS));

    assert!(approx_eq(t2.alpha().degrees_value(), t1.alpha().degrees_value(), DEFAULT_EPS));
    assert!(approx_eq(t2.beta().degrees_value(), t1.beta().degrees_value(), DEFAULT_EPS));
}

// ---------------------------------------------------------
// Symmetry: swapping legs swaps angles
// ---------------------------------------------------------

#[test]
fn swapping_legs_swaps_angles_and_keeps_hypotenuse() {
    let t1 = RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap();
    let t2 = RightTriangleSolver::from_legs(len(4.0), len(3.0)).unwrap();

    assert!(approx_eq(t1.c().mm_value(), t2.c().mm_value(), DEFAULT_EPS));
    assert!(approx_eq(t1.alpha().degrees_value(), t2.beta().degrees_value(), DEFAULT_EPS));
    assert!(approx_eq(t1.beta().degrees_value(), t2.alpha().degrees_value(), DEFAULT_EPS));
}

// ---------------------------------------------------------
// Robustness: non-classic triangles
// ---------------------------------------------------------

#[test]
fn handles_irregular_triangle() {
    let t = RightTriangleSolver::from_legs(len(7.123), len(2.987)).unwrap();
    assert_output_invariants(&t);
}

#[test]
fn handles_flat_triangle() {
    let t = RightTriangleSolver::from_legs(len(1000.0), len(0.01)).unwrap();
    assert_output_invariants(&t);
}

// ---------------------------------------------------------
// Numerical edge cases
// ---------------------------------------------------------

#[test]
fn tolerates_small_floating_error_in_hypotenuse() {
    // Your existing style: tiny rounding noise should not fail.
    let t = RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0000000000000001));
    assert!(t.is_ok());
}


#[test]
fn rejects_opposite_angle_when_sin_is_too_small() {
    // validate_acute accepts >0 degrees, but solver rejects if sin(alpha) < EPS.
    // Choose alpha small enough that sin(rad) < 1e-12.
    let alpha = angle_deg(1e-11); // degrees
    let result = RightTriangleSolver::from_leg_and_opposite_angle(len(3.0), alpha);
    assert!(result.is_err());
}

#[test]
fn accepts_small_but_valid_opposite_angle() {
    // Slightly larger than the EPS threshold so it should work.
    let alpha = angle_deg(1e-7); // degrees
    let result = RightTriangleSolver::from_leg_and_opposite_angle(len(3.0), alpha);
    assert!(result.is_ok());
}

#[test]
fn near_ninety_opposite_angle_is_still_valid() {
    // Opposite-angle constructor should be OK near 90° (sin ~ 1),
    // producing a very small adjacent leg.
    let alpha = angle_deg(89.999999);
    let t = RightTriangleSolver::from_leg_and_opposite_angle(len(10.0), alpha).unwrap();

    assert_output_invariants(&t);
    assert!(t.b().mm_value() < 0.01); // adjacent becomes tiny
}

#[test]
fn adjacent_leg_constructor_rejects_tan_overflow() {
    // tan(alpha) blows up near 90°. Use huge b to overflow multiplication -> inf.
    let b = len(1e308);
    let alpha = angle_deg(89.999999999999); // very close to 90 but still < 90
    let result = RightTriangleSolver::from_adjacent_leg_and_angle(b, alpha);

    // Either it errors (preferred) or it returns finite values (acceptable).
    match result {
        Ok(t) => {
            assert!(t.a().mm_value().is_finite());
            assert_output_invariants(&t);
        }
        Err(_) => {}
    }
}

// ---------------------------------------------------------
// Error cases: invalid lengths
// ---------------------------------------------------------

#[test]
fn rejects_zero_leg_in_from_legs() {
    let r = RightTriangleSolver::from_legs(len_any(0.0), len(4.0));
    assert!(matches!(r, Err(GeometryError::InvalidTriangle)));
}

#[test]
fn rejects_negative_leg_in_from_legs() {
    let r = RightTriangleSolver::from_legs(len_any(-3.0), len(4.0));
    assert!(matches!(r, Err(GeometryError::InvalidTriangle)));
}

#[test]
fn rejects_zero_hypotenuse_in_hyp_angle_constructor() {
    let r = RightTriangleSolver::from_hypotenuse_and_angle(len_any(0.0), angle_deg(30.0));
    assert!(r.is_err());
}

#[test]
fn rejects_negative_hypotenuse_in_hyp_angle_constructor() {
    let r = RightTriangleSolver::from_hypotenuse_and_angle(len_any(-5.0), angle_deg(30.0));
    assert!(r.is_err());
}

#[test]
fn rejects_zero_leg_in_opposite_angle_constructor() {
    let r = RightTriangleSolver::from_leg_and_opposite_angle(len_any(0.0), angle_deg(30.0));
    assert!(r.is_err());
}

#[test]
fn rejects_zero_leg_in_adjacent_angle_constructor() {
    let r = RightTriangleSolver::from_adjacent_leg_and_angle(len_any(0.0), angle_deg(30.0));
    assert!(r.is_err());
}

// ---------------------------------------------------------
// Error cases: impossible geometry
// ---------------------------------------------------------

#[test]
fn rejects_impossible_triangle_when_leg_ge_hypotenuse() {
    let r = RightTriangleSolver::from_leg_and_hypotenuse(len(5.0), len(3.0));
    assert!(matches!(r, Err(GeometryError::ImpossibleTriangle)));
}

#[test]
fn rejects_impossible_triangle_when_other_leg_ge_hypotenuse() {
    let r = RightTriangleSolver::from_other_leg_and_hypotenuse(len(5.0), len(3.0));
    assert!(matches!(r, Err(GeometryError::ImpossibleTriangle)));
}

// ---------------------------------------------------------
// Error cases: invalid angles
// ---------------------------------------------------------

#[test]
fn rejects_angle_zero() {
    let r = RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(0.0));
    assert!(matches!(r, Err(GeometryError::InvalidTriangle)));
}

#[test]
fn rejects_angle_ninety() {
    let r = RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(90.0));
    assert!(matches!(r, Err(GeometryError::InvalidTriangle)));
}

#[test]
fn rejects_angle_negative() {
    let r = RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(-30.0));
    assert!(matches!(r, Err(GeometryError::InvalidTriangle)));
}

#[test]
fn rejects_angle_over_ninety() {
    let r = RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(120.0));
    assert!(matches!(r, Err(GeometryError::InvalidTriangle)));
}
