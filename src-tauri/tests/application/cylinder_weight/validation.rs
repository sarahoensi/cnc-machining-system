use cnc_machining_system_lib::application::{
    CreateCylinderMaterialInput, CreateCylinderMaterialUseCase, CylinderMaterialRecord,
    CylinderMaterialRepository, SolveCylinderWeightInput, SolveCylinderWeightUseCase,
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

    fn create(&mut self, material: Material) -> Result<CylinderMaterialRecord, String> {
        if self
            .rows
            .iter()
            .any(|r| r.material.normalized_name() == material.normalized_name())
        {
            return Err("duplicate_material".to_string());
        }
        let rec = CylinderMaterialRecord {
            id: "new-id".to_string(),
            material,
        };
        self.rows.push(rec.clone());
        Ok(rec)
    }
}

#[test]
fn solve_fails_when_material_not_found() {
    let repo = FakeRepo { rows: vec![] };

    let result = SolveCylinderWeightUseCase::execute(
        &repo,
        SolveCylinderWeightInput {
            material_id: Some("missing".to_string()),
            outer_diameter_mm: Some(40.0),
            inner_diameter_mm: Some(0.0),
            length_mm: Some(100.0),
        },
    );

    assert!(result.is_err());
}

#[test]
fn solve_fails_when_required_fields_are_missing() {
    let repo = FakeRepo {
        rows: vec![CylinderMaterialRecord {
            id: "steel".to_string(),
            material: Material::new("Steel".to_string(), 7850.0).unwrap(),
        }],
    };

    let result = SolveCylinderWeightUseCase::execute(
        &repo,
        SolveCylinderWeightInput {
            material_id: None,
            outer_diameter_mm: None,
            inner_diameter_mm: Some(0.0),
            length_mm: None,
        },
    );

    assert!(result.is_err());
}

#[test]
fn create_fails_for_duplicate_material_name_case_insensitive() {
    let mut repo = FakeRepo {
        rows: vec![CylinderMaterialRecord {
            id: "steel".to_string(),
            material: Material::new("Steel".to_string(), 7850.0).unwrap(),
        }],
    };

    let result = CreateCylinderMaterialUseCase::execute(
        &mut repo,
        CreateCylinderMaterialInput {
            name: Some("steel".to_string()),
            density_kg_m3: Some(7850.0),
        },
    );

    assert!(result.is_err());
}
