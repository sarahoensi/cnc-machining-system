use cnc_machining_system_lib::application::{SolveCylinderWeightInput, SolveCylinderWeightOutput};
use cnc_machining_system_lib::interface::cylinder_weight::{
    SolveCylinderWeightRequest, SolveCylinderWeightResponse,
};

#[test]
fn request_maps_to_application_input() {
    let req = SolveCylinderWeightRequest {
        material_id: Some("steel".to_string()),
        outer_diameter_mm: Some(80.0),
        inner_diameter_mm: Some(10.0),
        length_mm: Some(350.0),
    };

    let input: SolveCylinderWeightInput = req.into();

    assert_eq!(input.material_id.as_deref(), Some("steel"));
    assert_eq!(input.outer_diameter_mm, Some(80.0));
    assert_eq!(input.inner_diameter_mm, Some(10.0));
    assert_eq!(input.length_mm, Some(350.0));
}

#[test]
fn application_output_maps_to_response() {
    let out = SolveCylinderWeightOutput {
        material_name: "Steel".to_string(),
        density_kg_m3: 7850.0,
        outer_diameter_mm: 50.0,
        inner_diameter_mm: 0.0,
        length_mm: 100.0,
        mass_kg: 1.542,
    };

    let response: SolveCylinderWeightResponse = out.into();

    assert_eq!(response.material_name, "Steel");
    assert_eq!(response.density_kg_m3, 7850.0);
    assert_eq!(response.outer_diameter_mm, 50.0);
    assert_eq!(response.inner_diameter_mm, 0.0);
    assert_eq!(response.length_mm, 100.0);
    assert_eq!(response.mass_kg, 1.542);
}
