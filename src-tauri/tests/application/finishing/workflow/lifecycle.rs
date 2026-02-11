// tests/application/finishing/workflow/lifecycle.rs

use cnc_machining_system_lib::application::finishing::use_cases::{
    RegisterFinishingMeasurementUseCase,
};

use super::super::fixtures::*;

#[test]
fn full_finishing_workflow() {
    let repo = repo();

    let register = RegisterFinishingMeasurementUseCase::new(repo.clone());

    let id = create_execution(repo.clone(), 3);

    // --- Step 1 measurement ---
    register.execute(id, 1, 9.6).unwrap();

    let after_step1 = repo.get(id).unwrap();
    assert_eq!(
        after_step1.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.6)
    );

    // --- Step 2 measurement ---
    register.execute(id, 2, 8.9).unwrap();

    let after_step2 = repo.get(id).unwrap();
    assert_eq!(
        after_step2.steps()[1].measurement().map(|d| d.mm_value()),
        Some(8.9)
    );

    // --- Edit step 2 ---
    register.execute(id, 2, 8.7).unwrap();

    let after_edit = repo.get(id).unwrap();

    // ✔ measurement updated
    assert_eq!(
        after_edit.steps()[1].measurement().map(|d| d.mm_value()),
        Some(8.7)
    );

    // ✔ step 1 unchanged
    assert_eq!(
        after_edit.steps()[0].measurement().map(|d| d.mm_value()),
        Some(9.6)
    );

    // ✔ target still reached
    let last_step = after_edit.steps().last().unwrap();
    let target = after_edit.plan().target().mm_value();

    assert!((last_step.planned_end().mm_value() - target).abs() < 1e-9);
}
