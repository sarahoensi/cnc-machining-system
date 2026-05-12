use std::{env, fs};

use cnc_machining_system_lib::application::{
    CylinderMaterialRepository, JsonCylinderMaterialRepository,
};
use cnc_machining_system_lib::domain::machining::Material;
use uuid::Uuid;

fn temp_file_path() -> std::path::PathBuf {
    let mut p = env::temp_dir();
    p.push(format!(
        "cylinder_materials_test_{}.json",
        Uuid::new_v4()
    ));
    p
}

#[test]
fn seeds_default_materials_on_first_init() {
    let path = temp_file_path();

    let repo = JsonCylinderMaterialRepository::load_or_initialize(path.clone()).unwrap();
    let items = repo.list();

    assert!(!items.is_empty());
    assert!(items.iter().any(|m| m.material.name() == "Steel"));

    let _ = fs::remove_file(path);
}

#[test]
fn persists_created_material_and_reloads() {
    let path = temp_file_path();

    let mut repo = JsonCylinderMaterialRepository::load_or_initialize(path.clone()).unwrap();
    let saved = repo
        .create(Material::new("Test Alloy".to_string(), 7777.0).unwrap())
        .unwrap();

    let reloaded = JsonCylinderMaterialRepository::load_or_initialize(path.clone()).unwrap();
    let found = reloaded.get_by_id(&saved.id);

    assert!(found.is_some());
    assert_eq!(found.unwrap().material.name(), "Test Alloy");

    let _ = fs::remove_file(path);
}

#[test]
fn rejects_duplicate_material_case_insensitive() {
    let path = temp_file_path();
    let mut repo = JsonCylinderMaterialRepository::load_or_initialize(path.clone()).unwrap();

    let result = repo.create(Material::new("steel".to_string(), 7900.0).unwrap());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "duplicate_material");

    let _ = fs::remove_file(path);
}
