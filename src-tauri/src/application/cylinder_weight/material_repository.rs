use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::machining::Material;

#[derive(Debug, Clone)]
pub struct CylinderMaterialRecord {
    pub id: String,
    pub material: Material,
}

pub trait CylinderMaterialRepository {
    fn list(&self) -> Vec<CylinderMaterialRecord>;
    fn get_by_id(&self, id: &str) -> Option<CylinderMaterialRecord>;
    fn get_by_normalized_name(&self, normalized_name: &str) -> Option<CylinderMaterialRecord>;
    fn create(&mut self, material: Material) -> Result<CylinderMaterialRecord, String>;
}

#[derive(Debug, Clone)]
pub struct JsonCylinderMaterialRepository {
    path: PathBuf,
    data: PersistedMaterials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedMaterials {
    schema_version: u32,
    materials: Vec<PersistedMaterial>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedMaterial {
    id: String,
    name: String,
    normalized_name: String,
    density_kg_m3: f64,
}

impl JsonCylinderMaterialRepository {
    pub fn load_or_initialize(path: PathBuf) -> Result<Self, String> {
        if !path.exists() {
            let seeded = PersistedMaterials {
                schema_version: 1,
                materials: default_seed_materials(),
            };
            write_data_file(&path, &seeded)?;
            return Ok(Self { path, data: seeded });
        }

        let raw = fs::read_to_string(&path).map_err(|e| format!("failed to read materials file: {e}"))?;
        let mut data: PersistedMaterials =
            serde_json::from_str(&raw).map_err(|e| format!("failed to parse materials file: {e}"))?;

        if data.schema_version != 1 {
            return Err(format!("unsupported schema_version: {}", data.schema_version));
        }

        if data.materials.is_empty() {
            data.materials = default_seed_materials();
            write_data_file(&path, &data)?;
        }

        Ok(Self { path, data })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn persist(&self) -> Result<(), String> {
        write_data_file(&self.path, &self.data)
    }
}

impl CylinderMaterialRepository for JsonCylinderMaterialRepository {
    fn list(&self) -> Vec<CylinderMaterialRecord> {
        self.data
            .materials
            .iter()
            .filter_map(|m| {
                let material = Material::new(m.name.clone(), m.density_kg_m3).ok()?;
                Some(CylinderMaterialRecord {
                    id: m.id.clone(),
                    material,
                })
            })
            .collect()
    }

    fn get_by_id(&self, id: &str) -> Option<CylinderMaterialRecord> {
        self.data.materials.iter().find(|m| m.id == id).and_then(|m| {
            Material::new(m.name.clone(), m.density_kg_m3)
                .ok()
                .map(|material| CylinderMaterialRecord {
                    id: m.id.clone(),
                    material,
                })
        })
    }

    fn get_by_normalized_name(&self, normalized_name: &str) -> Option<CylinderMaterialRecord> {
        self.data
            .materials
            .iter()
            .find(|m| m.normalized_name == normalized_name)
            .and_then(|m| {
                Material::new(m.name.clone(), m.density_kg_m3)
                    .ok()
                    .map(|material| CylinderMaterialRecord {
                        id: m.id.clone(),
                        material,
                    })
            })
    }

    fn create(&mut self, material: Material) -> Result<CylinderMaterialRecord, String> {
        if self
            .data
            .materials
            .iter()
            .any(|m| m.normalized_name == material.normalized_name())
        {
            return Err("duplicate_material".to_string());
        }

        let record = PersistedMaterial {
            id: Uuid::new_v4().to_string(),
            name: material.name().to_string(),
            normalized_name: material.normalized_name().to_string(),
            density_kg_m3: material.density_kg_m3(),
        };

        self.data.materials.push(record.clone());
        self.persist()?;

        Ok(CylinderMaterialRecord {
            id: record.id,
            material,
        })
    }
}

fn write_data_file(path: &Path, data: &PersistedMaterials) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("failed to create app data directory: {e}"))?;
    }

    let tmp_path = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(data).map_err(|e| format!("failed to serialize materials: {e}"))?;

    {
        let mut f = fs::File::create(&tmp_path).map_err(|e| format!("failed to create temp materials file: {e}"))?;
        f.write_all(&bytes).map_err(|e| format!("failed to write temp materials file: {e}"))?;
        f.flush().map_err(|e| format!("failed to flush temp materials file: {e}"))?;
    }

    fs::rename(&tmp_path, path).map_err(|e| format!("failed to replace materials file: {e}"))?;
    Ok(())
}

fn default_seed_materials() -> Vec<PersistedMaterial> {
    vec![
        ("Steel", 7850.0),
        ("Stainless Steel", 8000.0),
        ("Aluminum 6061", 2700.0),
        ("Brass", 8500.0),
        ("Copper", 8960.0),
        ("Titanium Grade 5", 4430.0),
        ("Delrin", 1410.0),
        ("Nylon", 1150.0),
    ]
    .into_iter()
    .map(|(name, density)| PersistedMaterial {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        normalized_name: Material::normalize_name(name),
        density_kg_m3: density,
    })
    .collect()
}
