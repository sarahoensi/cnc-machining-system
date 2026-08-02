use cnc_machining_system_lib::domain::machining::{
    CylinderSpec, CylinderWeightError, CylinderWeightSolver, Material,
};

#[test]
fn calculates_mass_for_solid_cylinder() {
    let spec = CylinderSpec::new(100.0, 0.0, 1000.0).unwrap();
    let steel = Material::new("Steel".to_string(), 7850.0).unwrap();

    let mass = CylinderWeightSolver::calculate_mass_kg(spec, &steel).unwrap();

    assert!((mass - 61.6537558).abs() < 1e-6);
}

#[test]
fn rejects_inner_diameter_larger_or_equal_outer() {
    let err = CylinderSpec::new(50.0, 50.0, 200.0).unwrap_err();
    assert_eq!(err, CylinderWeightError::InnerDiameterNotSmallerThanOuter);
}

#[test]
fn calculates_mass_for_hollow_cylinder() {
    let spec = CylinderSpec::new(80.0, 40.0, 500.0).unwrap();
    let aluminum = Material::new("Aluminum 6061".to_string(), 2700.0).unwrap();

    let mass = CylinderWeightSolver::calculate_mass_kg(spec, &aluminum).unwrap();

    assert!((mass - 5.0893801).abs() < 1e-6);
}

#[test]
fn mass_scales_linearly_with_length_and_density() {
    let base_spec = CylinderSpec::new(80.0, 40.0, 500.0).unwrap();
    let double_length_spec = CylinderSpec::new(80.0, 40.0, 1000.0).unwrap();
    let aluminum = Material::new("Aluminum 6061".to_string(), 2700.0).unwrap();
    let heavy_alloy = Material::new("Heavy Alloy".to_string(), 5400.0).unwrap();

    let base = CylinderWeightSolver::calculate_mass_kg(base_spec, &aluminum).unwrap();
    let doubled_length =
        CylinderWeightSolver::calculate_mass_kg(double_length_spec, &aluminum).unwrap();
    let doubled_density = CylinderWeightSolver::calculate_mass_kg(base_spec, &heavy_alloy).unwrap();

    assert!((doubled_length - base * 2.0).abs() < 1e-9);
    assert!((doubled_density - base * 2.0).abs() < 1e-9);
}

#[test]
fn rejects_non_positive_outer_diameter() {
    let err = CylinderSpec::new(0.0, 0.0, 100.0).unwrap_err();
    assert_eq!(err, CylinderWeightError::InvalidOuterDiameter);
}

#[test]
fn rejects_negative_inner_diameter() {
    let err = CylinderSpec::new(20.0, -1.0, 100.0).unwrap_err();
    assert_eq!(err, CylinderWeightError::InvalidInnerDiameter);
}

#[test]
fn rejects_non_positive_length() {
    let err = CylinderSpec::new(20.0, 0.0, 0.0).unwrap_err();
    assert_eq!(err, CylinderWeightError::InvalidLength);
}

#[test]
fn rejects_invalid_material_density_and_name() {
    let bad_name = Material::new("   ".to_string(), 7850.0).unwrap_err();
    assert_eq!(bad_name, CylinderWeightError::InvalidMaterialName);

    let bad_density = Material::new("Steel".to_string(), 0.0).unwrap_err();
    assert_eq!(bad_density, CylinderWeightError::InvalidDensity);
}
