// tests/domain/geometry/right_triangle_solver_tests.rs

use cnc_machining_system_lib::domain::RightTriangleSolver;
use cnc_machining_system_lib::domain::Length;
use cnc_machining_system_lib::domain::Angle;
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

fn len(v: f64) -> Length {
    Length::mm_positive(v).unwrap()
}

fn angle_deg(v: f64) -> Angle {
    Angle::degrees(v).unwrap()
}

// from legs
#[test]
fn from_legs_builds_correct_triangle() {
    let t = RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap();

    assert!(approx_eq(t.c().mm_value(), 5.0, DEFAULT_EPS));
}


// from leg + hypotenuse
#[test]
fn from_leg_and_hypotenuse_builds_correct_triangle() {
    let t = RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0)).unwrap();

    assert!(approx_eq(t.b().mm_value(), 4.0, DEFAULT_EPS));
}

#[test]
fn from_other_leg_and_hypotenuse_builds_correct_triangle() {
    let t = RightTriangleSolver::from_other_leg_and_hypotenuse(len(4.0), len(5.0)).unwrap();

    assert!(approx_eq(t.a().mm_value(), 3.0, DEFAULT_EPS));
}

// hyp + angle
#[test]
fn from_hypotenuse_and_angle_builds_correct_triangle() {
    let t = RightTriangleSolver::from_hypotenuse_and_angle(
        len(5.0),
        angle_deg(36.86989765),
    ).unwrap();

    assert!(approx_eq(t.a().mm_value(), 3.0, DEFAULT_EPS));
}

#[test]
fn from_leg_and_opposite_angle_builds_correct_triangle() {
    let t = RightTriangleSolver::from_leg_and_opposite_angle(
        len(3.0),
        angle_deg(36.86989765),
    ).unwrap();

    assert!(approx_eq(t.c().mm_value(), 5.0, DEFAULT_EPS));
}

#[test]
fn from_adjacent_leg_and_angle_builds_correct_triangle() {
    let t = RightTriangleSolver::from_adjacent_leg_and_angle(
        len(4.0),
        angle_deg(36.86989765),
    ).unwrap();

    assert!(approx_eq(t.a().mm_value(), 3.0, DEFAULT_EPS));
}

// cross-path concistency
#[test]
fn all_constructors_produce_same_triangle() {
    let t1 = RightTriangleSolver::from_legs(len(3.0), len(4.0)).unwrap();

    let t2 = RightTriangleSolver::from_leg_and_hypotenuse(len(3.0), len(5.0)).unwrap();

    assert!(approx_eq(
        t1.c().mm_value(),
        t2.c().mm_value(),
        DEFAULT_EPS
    ));
}

// error cases
#[test]
fn rejects_invalid_angles() {
    assert!(
        RightTriangleSolver::from_hypotenuse_and_angle(len(5.0), angle_deg(90.0))
            .is_err()
    );
}

#[test]
fn rejects_impossible_triangle() {
    assert!(
        RightTriangleSolver::from_leg_and_hypotenuse(len(5.0), len(3.0))
            .is_err()
    );
}
