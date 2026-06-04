// mapping.rs

use cnc_machining_system_lib::interface::tauri::helix::{HelixMode, SolveHelixRequest};

use cnc_machining_system_lib::application::SolveHelixInput;

#[test]
fn request_maps_to_application_input() {
    let request = SolveHelixRequest::Pitch {
        mode: HelixMode::Outer,
        diameter: 10.0,
        tool_diameter: 2.0,
        pitch: 4.0,
    };

    let input: SolveHelixInput = request.into();

    match input {
        SolveHelixInput::Pitch { diameter, .. } => {
            assert_eq!(diameter, 10.0);
        }
        _ => panic!("Wrong variant"),
    }
}
