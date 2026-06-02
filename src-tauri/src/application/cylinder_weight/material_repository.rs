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
    fn update(&mut self, id: &str, material: Material) -> Result<CylinderMaterialRecord, String>;
    fn delete(&mut self, id: &str) -> Result<(), String>;
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

        let raw =
            fs::read_to_string(&path).map_err(|e| format!("failed to read materials file: {e}"))?;
        let data: PersistedMaterials = serde_json::from_str(&raw)
            .map_err(|e| format!("failed to parse materials file: {e}"))?;

        if data.schema_version != 1 {
            return Err(format!(
                "unsupported schema_version: {}",
                data.schema_version
            ));
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
        self.data
            .materials
            .iter()
            .find(|m| m.id == id)
            .and_then(|m| {
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

    fn update(&mut self, id: &str, material: Material) -> Result<CylinderMaterialRecord, String> {
        if self
            .data
            .materials
            .iter()
            .any(|m| m.id != id && m.normalized_name == material.normalized_name())
        {
            return Err("duplicate_material".to_string());
        }

        let Some(row) = self.data.materials.iter_mut().find(|m| m.id == id) else {
            return Err("material_not_found".to_string());
        };

        row.name = material.name().to_string();
        row.normalized_name = material.normalized_name().to_string();
        row.density_kg_m3 = material.density_kg_m3();

        self.persist()?;

        Ok(CylinderMaterialRecord {
            id: id.to_string(),
            material,
        })
    }

    fn delete(&mut self, id: &str) -> Result<(), String> {
        let len_before = self.data.materials.len();
        self.data.materials.retain(|m| m.id != id);

        if self.data.materials.len() == len_before {
            return Err("material_not_found".to_string());
        }

        self.persist()?;
        Ok(())
    }
}

fn write_data_file(path: &Path, data: &PersistedMaterials) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create app data directory: {e}"))?;
    }

    let tmp_path = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(data)
        .map_err(|e| format!("failed to serialize materials: {e}"))?;

    {
        let mut f = fs::File::create(&tmp_path)
            .map_err(|e| format!("failed to create temp materials file: {e}"))?;
        f.write_all(&bytes)
            .map_err(|e| format!("failed to write temp materials file: {e}"))?;
        f.flush()
            .map_err(|e| format!("failed to flush temp materials file: {e}"))?;
    }

    fs::rename(&tmp_path, path).map_err(|e| format!("failed to replace materials file: {e}"))?;
    Ok(())
}

fn default_seed_materials() -> Vec<PersistedMaterial> {
    Vec::new()
}
