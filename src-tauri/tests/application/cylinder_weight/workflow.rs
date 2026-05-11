use cnc_machining_system_lib::application::{
    CylinderMaterialRecord, CylinderMaterialRepository, SolveCylinderWeightInput,
    SolveCylinderWeightUseCase,
};
use cnc_machining_system_lib::domain::machining::Material;

struct FakeRepo {
    rows: Vec<CylinderMaterialRecord>,
}

impl CylinderMaterialRepository for FakeRepo {
    fn list(&self) -> Vec<CylinderMaterialRecord> {
        self.rows.clone()
    }

    fn get_by_id(&self, id: &str) -> Option<CylinderMaterialRecord> {
        self.rows.iter().find(|r| r.id == id).cloned()
    }

    fn get_by_normalized_name(&self, normalized_name: &str) -> Option<CylinderMaterialRecord> {
        self.rows
            .iter()
            .find(|r| r.material.normalized_name() == normalized_name)
            .cloned()
    }

    fn create(&mut self, _material: Material) -> Result<CylinderMaterialRecord, String> {
        unreachable!()
    }
}

#[test]
fn solve_returns_mass_rounded_to_3_decimals() {
    let repo = FakeRepo {
        rows: vec![CylinderMaterialRecord {
            id: "mat-steel".to_string(),
            material: Material::new("Steel".to_string(), 7850.0).unwrap(),
        }],
    };

    let input = SolveCylinderWeightInput {
        material_id: Some("mat-steel".to_string()),
        outer_diameter_mm: Some(100.0),
        inner_diameter_mm: Some(0.0),
        length_mm: Some(1000.0),
    };

    let out = SolveCylinderWeightUseCase::execute(&repo, input).unwrap();
    assert_eq!(out.mass_kg, 61.654);
}
