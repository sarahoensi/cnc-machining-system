use cnc_machining_system_lib::interface::cylinder_weight::{
    CreateCylinderMaterialRequest, SolveCylinderWeightRequest,
};

#[test]
fn deserializes_solve_request_json() {
    let json = r#"
    {
      "material_id": "steel",
      "outer_diameter_mm": 50.0,
      "inner_diameter_mm": 0.0,
      "length_mm": 100.0
    }
    "#;

    let req: SolveCylinderWeightRequest = serde_json::from_str(json).unwrap();

    assert_eq!(req.material_id.as_deref(), Some("steel"));
    assert_eq!(req.outer_diameter_mm, Some(50.0));
    assert_eq!(req.inner_diameter_mm, Some(0.0));
    assert_eq!(req.length_mm, Some(100.0));
}

#[test]
fn deserializes_create_material_request_json() {
    let json = r#"
    {
      "name": "Bronze",
      "density_kg_m3": 8800.0
    }
    "#;

    let req: CreateCylinderMaterialRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.name.as_deref(), Some("Bronze"));
    assert_eq!(req.density_kg_m3, Some(8800.0));
}
