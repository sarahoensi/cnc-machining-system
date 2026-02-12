// 

use cnc_machining_system_lib::interface::tauri::helix::{
    SolveHelixRequest,
    HelixMode,
};

use cnc_machining_system_lib::application::SolveHelixInput;

#[test]
fn request_maps_to_application_input() {

    let request = SolveHelixRequest::Pitch {
        mode: HelixMode::Outer,
        diameter_mm: 10.0,
        tool_diameter_mm: 2.0,
        pitch_mm_per_rev: 4.0,
    };

    let input: SolveHelixInput = request.into();

    match input {
        SolveHelixInput::Pitch { diameter_mm, .. } => {
            assert_eq!(diameter_mm, 10.0);
        }
        _ => panic!("Wrong variant"),
    }
}
