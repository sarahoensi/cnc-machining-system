// tests/domain/machining_strategy/finishing_planner_tests.rs

use cnc_machining_system_lib::domain::{
    units::*,
    machining::finishing::{
        FinishingPlanner,
        FinishingPlanning,
        FinishingRequest,
        FinishingMode,
    },
};

const EPS: f64 = 1e-9;

fn d(v: f64) -> Diameter {
    Diameter::mm(v).unwrap()
}

fn l(v: f64) -> PositiveLength {
    PositiveLength::mm(v).unwrap()
}

//
// ----------------------------------------------
// ByCuts tests
// ----------------------------------------------

#[test]
fn bycuts_generates_correct_step() {
    let req = FinishingRequest {
        mode: FinishingMode::Outer,
        start_diameter: d(10.0),
        target_diameter: d(8.0),
        planning: FinishingPlanning::ByCuts(2),
    };

    let plan = FinishingPlanner::generate_plan(req).unwrap();

    assert_eq!(plan.cuts(), 2);

    assert!((plan.expected_step().mm_value() - 1.0).abs() < EPS);
}

//
// ----------------------------------------------
// Radial engagement planning
// ----------------------------------------------

#[test]
fn radial_engagement_computes_cuts() {
    let req = FinishingRequest {
        mode: FinishingMode::Outer,
        start_diameter: d(10.0),
        target_diameter: d(8.0),
        planning: FinishingPlanning::ByRadialEngagement(l(0.5)),
    };

    let plan = FinishingPlanner::generate_plan(req).unwrap();

    assert_eq!(plan.cuts(), 2);
}

//
// ----------------------------------------------
// Direction validation
// ----------------------------------------------

#[test]
fn rejects_wrong_direction_inner() {
    let req = FinishingRequest {
        mode: FinishingMode::Inner,
        start_diameter: d(10.0),
        target_diameter: d(9.0),
        planning: FinishingPlanning::ByCuts(2),
    };

    assert!(FinishingPlanner::generate_plan(req).is_err());
}

//
// ----------------------------------------------
// Edge cases
// ----------------------------------------------

#[test]
fn rejects_zero_delta() {
    let req = FinishingRequest {
        mode: FinishingMode::Outer,
        start_diameter: d(10.0),
        target_diameter: d(10.0),
        planning: FinishingPlanning::ByCuts(2),
    };

    assert!(FinishingPlanner::generate_plan(req).is_err());
}