use cnc_machining_system_lib::application::{SolveHelixInput, SolveHelixUseCase};
use cnc_machining_system_lib::domain::HelixMode;

#[test]
fn outer_mode_adds_tool_radius() {

    let uc = SolveHelixUseCase;

    let result = uc.execute(
        SolveHelixInput::Pitch {
            mode: HelixMode::Outer,
            diameter_mm: 10.0,
            tool_diameter_mm: 2.0,
            pitch_mm_per_rev: 4.0,
        }
    ).unwrap();

    assert_eq!(result.effective_diameter_mm, 11.0);
}

#[test]
fn inner_mode_subtracts_tool_radius() {

    let uc = SolveHelixUseCase;

    let result = uc.execute(
        SolveHelixInput::Pitch {
            mode: HelixMode::Inner,
            diameter_mm: 10.0,
            tool_diameter_mm: 2.0,
            pitch_mm_per_rev: 4.0,
        }
    ).unwrap();

    assert_eq!(result.effective_diameter_mm, 9.0);
}
