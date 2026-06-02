// tests/interface/tauri/right_triangle/mapping.rs

use cnc_machining_system_lib::interface::right_triangle::{
    SolveRightTriangleRequest, SolveRightTriangleResponse,
};

use cnc_machining_system_lib::application::{SolveRightTriangleInput, SolveRightTriangleOutput};

#[test]
fn maps_legs_request_to_application_input() {
    let request = SolveRightTriangleRequest::Legs {
        a_mm: 3.0,
        b_mm: 4.0,
    };

    let input: SolveRightTriangleInput = request.into();

    match input {
        SolveRightTriangleInput::Legs { a_mm, b_mm } => {
            assert_eq!(a_mm, 3.0);
            assert_eq!(b_mm, 4.0);
        }
        _ => panic!("wrong mapping"),
    }
}

#[test]
fn maps_application_output_to_response() {
    let output = SolveRightTriangleOutput {
        a_mm: 3.0,
        b_mm: 4.0,
        c_mm: 5.0,
        alpha_deg: 36.87,
        beta_deg: 53.13,
    };

    let response: SolveRightTriangleResponse = output.into();

    assert_eq!(response.c_mm, 5.0);
}
