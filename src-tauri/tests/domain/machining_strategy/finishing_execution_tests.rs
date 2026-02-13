// tests/domain/machining_strategy/finishing_execution_tests.rs

use cnc_machining_system_lib::domain::{units::*,*};

const EPS: f64 = 1e-9;

//
// ----------------------------------------------------
// Helpers
// ----------------------------------------------------
//

fn d(v: f64) -> Diameter {
    Diameter::mm(v).unwrap()
}

fn exec(plan: FinishingPlan) -> FinishingExecution {
    FinishingExecution::new(FinishingExecutionId::new(), plan).unwrap()
}



fn outer_plan(start: f64, target: f64, cuts: u32) -> FinishingPlan {
    FinishingPlanner::generate_plan(FinishingRequest {
        mode: FinishingMode::Outer,
        start_diameter: d(start),
        target_diameter: d(target),
        planning: FinishingPlanning::ByCuts(cuts),
    })
    .unwrap()
}

fn inner_plan(start: f64, target: f64, cuts: u32) -> FinishingPlan {
    FinishingPlanner::generate_plan(FinishingRequest {
        mode: FinishingMode::Inner,
        start_diameter: d(start),
        target_diameter: d(target),
        planning: FinishingPlanning::ByCuts(cuts),
    })
    .unwrap()
}

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

fn assert_execution_invariants(exec: &FinishingExecution) {
    let plan = exec.plan();
    let steps = exec.steps();

    assert!(!steps.is_empty());

    // first step must start at plan.start
    assert!(approx(
        steps[0].start().mm_value(),
        plan.start().mm_value()
    ));

    // last step must end at target
    let last = steps.last().unwrap();
    assert!(approx(
        last.planned_end().mm_value(),
        plan.target().mm_value()
    ));

    // steps must be monotonic
    let dir = plan.direction_sign();

    for s in steps.windows(2) {
        let a = s[0].planned_end().mm_value();
        let b = s[1].planned_end().mm_value();

        if dir > 0.0 {
            assert!(b > a);
        } else {
            assert!(b < a);
        }
    }
}

//
// ======================================================
// Step building
// ======================================================
//

#[test]
fn builds_correct_number_of_steps() {
    let exec = exec(outer_plan(10.0, 8.0, 2));

    assert_eq!(exec.steps().len(), 2);
}

#[test]
fn builds_steps_reaching_target() {
    let exec = exec(outer_plan(10.0, 8.0, 2));
    assert_execution_invariants(&exec);
}

//
// ======================================================
// Measurement workflow
// ======================================================
//

#[test]
fn register_measurement_stores_value() {
    let mut exec = exec(outer_plan(10.0, 8.0, 2));

    exec.register_measurement(1, d(9.3)).unwrap();

    assert!(exec.steps()[0].measurement().is_some());
}

//
// ======================================================
// Adaptive recalculation
// ======================================================
//

#[test]
fn measurement_recalculates_remaining_steps() {
    let mut exec = exec(outer_plan(10.0, 8.0, 2));

    exec.register_measurement(1, d(9.5)).unwrap();

    let last = exec.steps().last().unwrap();
    assert!(approx(last.planned_end().mm_value(), 8.0));
}

#[test]
fn recalculation_redistributes_remaining_delta() {
    let mut exec = exec(outer_plan(10.0, 8.0, 4));

    exec.register_measurement(1, d(9.7)).unwrap();

    assert_execution_invariants(&exec);
}

//
// ======================================================
// Locking rules
// ======================================================
//

#[test]
fn cannot_edit_earlier_step_when_later_measured() {
    let mut exec = exec(outer_plan(10.0, 8.0, 3));

    exec.register_measurement(2, d(8.7)).unwrap();

    let result = exec.register_measurement(1, d(9.3));

    assert!(result.is_err());
}



//
// ======================================================
// Direction behaviour
// ======================================================
//

#[test]
fn outer_mode_decreases_diameter() {
    let exec = exec(outer_plan(10.0, 8.0, 3));
    assert_execution_invariants(&exec);
}

#[test]
fn inner_mode_increases_diameter() {
    let exec = exec(inner_plan(8.0, 10.0, 3));
    assert_execution_invariants(&exec);
}

//
// ======================================================
// Overshoot validation
// ======================================================
//

#[test]
fn rejects_measurement_past_target_outer() {
    let mut exec = exec(outer_plan(10.0, 8.0, 2));

    let r = exec.register_measurement(1, d(7.0));
    assert!(r.is_err());
}

#[test]
fn rejects_measurement_past_target_inner() {
    let mut exec = exec(inner_plan(8.0, 10.0, 2));

    let r = exec.register_measurement(1, d(11.0));
    assert!(r.is_err());
}

//
// ======================================================
// Index validation
// ======================================================
//

#[test]
fn rejects_step_zero() {
    let mut exec = exec(outer_plan(10.0, 8.0, 2));

    assert!(exec.register_measurement(0, d(9.0)).is_err());
}

#[test]
fn rejects_step_out_of_range() {
    let mut exec = exec(outer_plan(10.0, 8.0, 2));

    assert!(exec.register_measurement(5, d(9.0)).is_err());
}
