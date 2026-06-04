use crate::{
    application::shared::{AppResult, ApplicationError, InputParser},
    domain::machining::{CylinderSpec, CylinderWeightError, CylinderWeightSolver, Material},
};
use serde::Deserialize;
use std::collections::HashSet;

use super::{
    CreateCylinderMaterialInput, CylinderMaterialOutput, CylinderMaterialRepository,
    DeleteCylinderMaterialInput, ExportCylinderMaterialRow, ExportCylinderMaterialsOutput,
    ImportAddedMaterialRow, ImportCylinderMaterialsInput, ImportCylinderMaterialsOutput,
    ImportSkippedMaterialRow, SolveCylinderWeightInput, SolveCylinderWeightOutput,
    UpdateCylinderMaterialInput,
};

pub struct ListCylinderMaterialsUseCase;
pub struct CreateCylinderMaterialUseCase;
pub struct UpdateCylinderMaterialUseCase;
pub struct DeleteCylinderMaterialUseCase;
pub struct ImportCylinderMaterialsUseCase;
pub struct ExportCylinderMaterialsUseCase;
pub struct SolveCylinderWeightUseCase;

impl ListCylinderMaterialsUseCase {
    pub fn execute(repo: &dyn CylinderMaterialRepository) -> Vec<CylinderMaterialOutput> {
        repo.list()
            .into_iter()
            .map(|r| CylinderMaterialOutput {
                id: r.id,
                name: r.material.name().to_string(),
                density_kg_m3: r.material.density_kg_m3(),
            })
            .collect()
    }
}

impl CreateCylinderMaterialUseCase {
    pub fn execute(
        repo: &mut dyn CylinderMaterialRepository,
        input: CreateCylinderMaterialInput,
    ) -> AppResult<CylinderMaterialOutput> {
        let mut p = InputParser::new();

        let name = match input.name {
            Some(v) => Some(v),
            None => {
                p.push("name", "validation_error", "is required");
                None
            }
        };
        let density = match input.density_kg_m3 {
            Some(v) => Some(v),
            None => {
                p.push("density_kg_m3", "validation_error", "is required");
                None
            }
        };

        let material = match (name, density) {
            (Some(n), Some(d)) => match Material::new(n, d) {
                Ok(m) => Some(m),
                Err(CylinderWeightError::InvalidMaterialName) => {
                    p.push(
                        "name",
                        "invalid",
                        CylinderWeightError::InvalidMaterialName.to_string(),
                    );
                    None
                }
                Err(CylinderWeightError::InvalidDensity) => {
                    p.push(
                        "density_kg_m3",
                        "invalid",
                        CylinderWeightError::InvalidDensity.to_string(),
                    );
                    None
                }
                Err(e) => {
                    p.push("name", "invalid", e.to_string());
                    None
                }
            },
            _ => None,
        };

        let material = p.finish_with(material)?;

        match repo.create(material) {
            Ok(saved) => Ok(CylinderMaterialOutput {
                id: saved.id,
                name: saved.material.name().to_string(),
                density_kg_m3: saved.material.density_kg_m3(),
            }),
            Err(e) if e == "duplicate_material" => Err(ApplicationError::Infrastructure(
                "duplicate_material".to_string(),
            )),
            Err(e) => Err(ApplicationError::Infrastructure(e)),
        }
    }
}

impl SolveCylinderWeightUseCase {
    pub fn execute(
        repo: &dyn CylinderMaterialRepository,
        input: SolveCylinderWeightInput,
    ) -> AppResult<SolveCylinderWeightOutput> {
        let mut p = InputParser::new();

        let material_id = match input.material_id {
            Some(v) if !v.trim().is_empty() => Some(v),
            Some(_) => {
                p.push("material_id", "validation_error", "must not be empty");
                None
            }
            None => {
                p.push("material_id", "validation_error", "is required");
                None
            }
        };

        let outer = match input.outer_diameter_mm {
            Some(v) => Some(v),
            None => {
                p.push("outer_diameter_mm", "validation_error", "is required");
                None
            }
        };
        let inner = Some(input.inner_diameter_mm.unwrap_or(0.0));
        let length = match input.length_mm {
            Some(v) => Some(v),
            None => {
                p.push("length_mm", "validation_error", "is required");
                None
            }
        };

        let material = material_id
            .as_deref()
            .and_then(|id| repo.get_by_id(id))
            .map(|r| (r.id, r.material));

        if material_id.is_some() && material.is_none() {
            p.push(
                "material_id",
                "material_not_found",
                "material was not found",
            );
        }

        let spec = match (outer, inner, length) {
            (Some(o), Some(i), Some(l)) => match CylinderSpec::new(o, i, l) {
                Ok(spec) => Some(spec),
                Err(CylinderWeightError::InvalidOuterDiameter) => {
                    p.push(
                        "outer_diameter_mm",
                        "invalid_geometry",
                        CylinderWeightError::InvalidOuterDiameter.to_string(),
                    );
                    None
                }
                Err(CylinderWeightError::InvalidInnerDiameter) => {
                    p.push(
                        "inner_diameter_mm",
                        "invalid_geometry",
                        CylinderWeightError::InvalidInnerDiameter.to_string(),
                    );
                    None
                }
                Err(CylinderWeightError::InnerDiameterNotSmallerThanOuter) => {
                    p.push(
                        "inner_diameter_mm",
                        "invalid_geometry",
                        CylinderWeightError::InnerDiameterNotSmallerThanOuter.to_string(),
                    );
                    None
                }
                Err(CylinderWeightError::InvalidLength) => {
                    p.push(
                        "length_mm",
                        "invalid_geometry",
                        CylinderWeightError::InvalidLength.to_string(),
                    );
                    None
                }
                Err(e) => {
                    p.push("inner_diameter_mm", "invalid_geometry", e.to_string());
                    None
                }
            },
            _ => None,
        };

        p.finish()?;

        let (_, material) = material.ok_or_else(|| {
            ApplicationError::Infrastructure(
                "material missing after successful validation".to_string(),
            )
        })?;
        let spec = spec.ok_or_else(|| {
            ApplicationError::Infrastructure("spec missing after successful validation".to_string())
        })?;

        let mass = CylinderWeightSolver::calculate_mass_kg(spec, &material)
            .map_err(|e| ApplicationError::Domain(e.into()))?;

        Ok(SolveCylinderWeightOutput {
            material_name: material.name().to_string(),
            density_kg_m3: material.density_kg_m3(),
            outer_diameter_mm: spec.outer_diameter_mm(),
            inner_diameter_mm: spec.inner_diameter_mm(),
            length_mm: spec.length_mm(),
            mass_kg: round3(mass),
        })
    }
}

fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

impl UpdateCylinderMaterialUseCase {
    pub fn execute(
        repo: &mut dyn CylinderMaterialRepository,
        input: UpdateCylinderMaterialInput,
    ) -> AppResult<CylinderMaterialOutput> {
        let mut p = InputParser::new();

        let id = match input.id {
            Some(v) if !v.trim().is_empty() => Some(v),
            Some(_) => {
                p.push("id", "validation_error", "must not be empty");
                None
            }
            None => {
                p.push("id", "validation_error", "is required");
                None
            }
        };

        let name = match input.name {
            Some(v) => Some(v),
            None => {
                p.push("name", "validation_error", "is required");
                None
            }
        };
        let density = match input.density_kg_m3 {
            Some(v) => Some(v),
            None => {
                p.push("density_kg_m3", "validation_error", "is required");
                None
            }
        };

        let material = match (name, density) {
            (Some(n), Some(d)) => match Material::new(n, d) {
                Ok(m) => Some(m),
                Err(CylinderWeightError::InvalidMaterialName) => {
                    p.push(
                        "name",
                        "invalid",
                        CylinderWeightError::InvalidMaterialName.to_string(),
                    );
                    None
                }
                Err(CylinderWeightError::InvalidDensity) => {
                    p.push(
                        "density_kg_m3",
                        "invalid",
                        CylinderWeightError::InvalidDensity.to_string(),
                    );
                    None
                }
                Err(e) => {
                    p.push("name", "invalid", e.to_string());
                    None
                }
            },
            _ => None,
        };

        p.finish()?;

        let id = id.ok_or_else(|| {
            ApplicationError::Infrastructure("id missing after validation".to_string())
        })?;
        let material = material.ok_or_else(|| {
            ApplicationError::Infrastructure("material missing after validation".to_string())
        })?;

        match repo.update(&id, material) {
            Ok(saved) => Ok(CylinderMaterialOutput {
                id: saved.id,
                name: saved.material.name().to_string(),
                density_kg_m3: saved.material.density_kg_m3(),
            }),
            Err(e) if e == "duplicate_material" => Err(ApplicationError::Infrastructure(
                "duplicate_material".to_string(),
            )),
            Err(e) if e == "material_not_found" => Err(ApplicationError::Infrastructure(
                "material_not_found".to_string(),
            )),
            Err(e) => Err(ApplicationError::Infrastructure(e)),
        }
    }
}

impl DeleteCylinderMaterialUseCase {
    pub fn execute(
        repo: &mut dyn CylinderMaterialRepository,
        input: DeleteCylinderMaterialInput,
    ) -> AppResult<()> {
        let mut p = InputParser::new();
        let id = match input.id {
            Some(v) if !v.trim().is_empty() => Some(v),
            Some(_) => {
                p.push("id", "validation_error", "must not be empty");
                None
            }
            None => {
                p.push("id", "validation_error", "is required");
                None
            }
        };
        p.finish()?;
        let id = id.ok_or_else(|| {
            ApplicationError::Infrastructure("id missing after validation".to_string())
        })?;

        match repo.delete(&id) {
            Ok(()) => Ok(()),
            Err(e) if e == "material_not_found" => Err(ApplicationError::Infrastructure(
                "material_not_found".to_string(),
            )),
            Err(e) => Err(ApplicationError::Infrastructure(e)),
        }
    }
}

#[derive(Deserialize)]
struct ImportPayload {
    schema_version: u32,
    materials: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct ImportMaterialRow {
    name: Option<String>,
    density_kg_m3: Option<f64>,
}

impl ImportCylinderMaterialsUseCase {
    pub fn execute(
        repo: &mut dyn CylinderMaterialRepository,
        input: ImportCylinderMaterialsInput,
    ) -> AppResult<ImportCylinderMaterialsOutput> {
        let mut p = InputParser::new();

        let payload = match input.json_payload {
            Some(v) if !v.trim().is_empty() => Some(v),
            Some(_) => {
                p.push("json_payload", "validation_error", "must not be empty");
                None
            }
            None => {
                p.push("json_payload", "validation_error", "is required");
                None
            }
        };

        p.finish()?;
        let payload = payload.ok_or_else(|| {
            ApplicationError::Infrastructure("json payload missing after validation".to_string())
        })?;

        let parsed: ImportPayload = serde_json::from_str(&payload)
            .map_err(|e| ApplicationError::Infrastructure(format!("invalid import json: {e}")))?;

        if parsed.schema_version != 1 {
            return Err(ApplicationError::Infrastructure(format!(
                "unsupported schema_version: {}",
                parsed.schema_version
            )));
        }

        let mut imported = 0usize;
        let mut skipped_duplicates = 0usize;
        let mut skipped_invalid = 0usize;
        let mut added = Vec::<ImportAddedMaterialRow>::new();
        let mut skipped = Vec::<ImportSkippedMaterialRow>::new();

        let existing = repo.list();
        let mut used_names: HashSet<String> = existing
            .iter()
            .map(|row| row.material.normalized_name().to_string())
            .collect();
        let mut existing_pairs: HashSet<(String, u64)> = existing
            .iter()
            .map(|row| {
                (
                    row.material.normalized_name().to_string(),
                    row.material.density_kg_m3().to_bits(),
                )
            })
            .collect();

        for raw_row in parsed.materials {
            let row: ImportMaterialRow = match serde_json::from_value(raw_row.clone()) {
                Ok(v) => v,
                Err(_) => {
                    skipped_invalid += 1;
                    skipped.push(ImportSkippedMaterialRow {
                        name: None,
                        density_kg_m3: None,
                        reason: "invalid".to_string(),
                        message: "Invalid material row shape.".to_string(),
                    });
                    continue;
                }
            };
            let raw_name = row.name.clone();
            let raw_density = row.density_kg_m3;

            let Some(name) = row.name else {
                skipped_invalid += 1;
                skipped.push(ImportSkippedMaterialRow {
                    name: raw_name,
                    density_kg_m3: raw_density,
                    reason: "invalid".to_string(),
                    message: "Material name is required.".to_string(),
                });
                continue;
            };
            let Some(density) = row.density_kg_m3 else {
                skipped_invalid += 1;
                skipped.push(ImportSkippedMaterialRow {
                    name: Some(name),
                    density_kg_m3: raw_density,
                    reason: "invalid".to_string(),
                    message: "Density is required.".to_string(),
                });
                continue;
            };

            let material = match Material::new(name.clone(), density) {
                Ok(m) => m,
                Err(CylinderWeightError::InvalidMaterialName) => {
                    skipped_invalid += 1;
                    skipped.push(ImportSkippedMaterialRow {
                        name: Some(name),
                        density_kg_m3: Some(density),
                        reason: "invalid".to_string(),
                        message: "Material name is required.".to_string(),
                    });
                    continue;
                }
                Err(CylinderWeightError::InvalidDensity) => {
                    skipped_invalid += 1;
                    skipped.push(ImportSkippedMaterialRow {
                        name: Some(name),
                        density_kg_m3: Some(density),
                        reason: "invalid".to_string(),
                        message: "Density must be a positive number.".to_string(),
                    });
                    continue;
                }
                Err(e) => {
                    skipped_invalid += 1;
                    skipped.push(ImportSkippedMaterialRow {
                        name: Some(name),
                        density_kg_m3: Some(density),
                        reason: "invalid".to_string(),
                        message: e.to_string(),
                    });
                    continue;
                }
            };

            let normalized = material.normalized_name().to_string();
            let density_bits = material.density_kg_m3().to_bits();
            let pair_key = (normalized.clone(), density_bits);
            if existing_pairs.contains(&pair_key) {
                skipped_duplicates += 1;
                skipped.push(ImportSkippedMaterialRow {
                    name: Some(material.name().to_string()),
                    density_kg_m3: Some(material.density_kg_m3()),
                    reason: "duplicate".to_string(),
                    message: "Same name and density already exists.".to_string(),
                });
                continue;
            }

            let mut imported_material = material.clone();
            let mut original_name: Option<String> = None;
            if used_names.contains(&normalized) {
                match resolve_suffix_import_name(
                    material.name(),
                    material.density_kg_m3(),
                    &used_names,
                    &existing_pairs,
                ) {
                    SuffixResolution::DuplicateExistingName(existing_name) => {
                        skipped_duplicates += 1;
                        skipped.push(ImportSkippedMaterialRow {
                            name: Some(material.name().to_string()),
                            density_kg_m3: Some(material.density_kg_m3()),
                            reason: "duplicate".to_string(),
                            message: format!(
                                "Already exists as {existing_name} with same density."
                            ),
                        });
                        continue;
                    }
                    SuffixResolution::UniqueName(unique_name) => {
                        original_name = Some(material.name().to_string());
                        imported_material = Material::new(unique_name, material.density_kg_m3())
                            .map_err(|e| {
                                ApplicationError::Infrastructure(format!(
                                    "failed to rename imported material: {e}"
                                ))
                            })?;
                    }
                }
            }

            match repo.create(imported_material.clone()) {
                Ok(_) => {
                    imported += 1;
                    used_names.insert(imported_material.normalized_name().to_string());
                    existing_pairs.insert((
                        imported_material.normalized_name().to_string(),
                        imported_material.density_kg_m3().to_bits(),
                    ));
                    added.push(ImportAddedMaterialRow {
                        name: imported_material.name().to_string(),
                        density_kg_m3: imported_material.density_kg_m3(),
                        original_name,
                    });
                }
                Err(e) if e == "duplicate_material" => {
                    skipped_duplicates += 1;
                    skipped.push(ImportSkippedMaterialRow {
                        name: Some(imported_material.name().to_string()),
                        density_kg_m3: Some(imported_material.density_kg_m3()),
                        reason: "duplicate".to_string(),
                        message: "Material name already exists.".to_string(),
                    });
                }
                Err(e) => {
                    skipped_invalid += 1;
                    skipped.push(ImportSkippedMaterialRow {
                        name: Some(imported_material.name().to_string()),
                        density_kg_m3: Some(imported_material.density_kg_m3()),
                        reason: "invalid".to_string(),
                        message: format!("Failed to import material: {e}"),
                    });
                }
            }
        }

        Ok(ImportCylinderMaterialsOutput {
            imported,
            skipped_duplicates,
            skipped_invalid,
            added,
            skipped,
        })
    }
}

enum SuffixResolution {
    DuplicateExistingName(String),
    UniqueName(String),
}

fn resolve_suffix_import_name(
    base_name: &str,
    density_kg_m3: f64,
    used_names: &HashSet<String>,
    existing_pairs: &HashSet<(String, u64)>,
) -> SuffixResolution {
    let trimmed = base_name.trim();
    let mut suffix = 1usize;
    let density_bits = density_kg_m3.to_bits();

    loop {
        let candidate = format!("{trimmed} ({suffix})");
        let normalized = Material::normalize_name(&candidate);

        if existing_pairs.contains(&(normalized.clone(), density_bits)) {
            return SuffixResolution::DuplicateExistingName(candidate);
        }

        if !used_names.contains(&normalized) {
            return SuffixResolution::UniqueName(candidate);
        }

        suffix += 1;
    }
}

impl ExportCylinderMaterialsUseCase {
    pub fn execute(repo: &dyn CylinderMaterialRepository) -> ExportCylinderMaterialsOutput {
        let materials = repo
            .list()
            .into_iter()
            .map(|r| ExportCylinderMaterialRow {
                name: r.material.name().to_string(),
                density_kg_m3: r.material.density_kg_m3(),
            })
            .collect();

        ExportCylinderMaterialsOutput {
            schema_version: 1,
            materials,
        }
    }
}
