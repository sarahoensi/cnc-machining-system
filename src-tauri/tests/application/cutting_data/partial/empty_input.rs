// tests/application/cutting_data/partial/empty_input.rs

use cnc_machining_system_lib::application::{SolveCuttingDataInput, SolveCuttingDataUseCase};

#[test]
fn returns_empty_solution_when_no_inputs_given() {

    let output = SolveCuttingDataUseCase::execute(
        SolveCuttingDataInput::default()
    ).unwrap();

    assert!(output.rpm.is_none());
    assert!(output.feed_rate_mm_per_min.is_none());
}
