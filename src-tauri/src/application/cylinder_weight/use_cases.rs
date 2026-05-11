use crate::{
    application::{
        shared::{AppResult, ApplicationError, InputParser},
    },
    domain::machining::{CylinderSpec, CylinderWeightError, CylinderWeightSolver, Material},
};

use super::{
    CreateCylinderMaterialInput, CylinderMaterialOutput, CylinderMaterialRepository,
    DeleteCylinderMaterialInput, UpdateCylinderMaterialInput,
    SolveCylinderWeightInput, SolveCylinderWeightOutput,
};

pub struct ListCylinderMaterialsUseCase;
pub struct CreateCylinderMaterialUseCase;
pub struct UpdateCylinderMaterialUseCase;
pub struct DeleteCylinderMaterialUseCase;
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
                    p.push("name", "invalid", CylinderWeightError::InvalidMaterialName.to_string());
                    None
                }
                Err(CylinderWeightError::InvalidDensity) => {
                    p.push("density_kg_m3", "invalid", CylinderWeightError::InvalidDensity.to_string());
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
            p.push("material_id", "material_not_found", "material was not found");
        }

        let spec = match (outer, inner, length) {
            (Some(o), Some(i), Some(l)) => match CylinderSpec::new(o, i, l) {
                Ok(spec) => Some(spec),
                Err(CylinderWeightError::InvalidOuterDiameter) => {
                    p.push("outer_diameter_mm", "invalid_geometry", CylinderWeightError::InvalidOuterDiameter.to_string());
                    None
                }
                Err(CylinderWeightError::InvalidInnerDiameter) => {
                    p.push("inner_diameter_mm", "invalid_geometry", CylinderWeightError::InvalidInnerDiameter.to_string());
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
                    p.push("length_mm", "invalid_geometry", CylinderWeightError::InvalidLength.to_string());
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
            ApplicationError::Infrastructure("material missing after successful validation".to_string())
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
                    p.push("name", "invalid", CylinderWeightError::InvalidMaterialName.to_string());
                    None
                }
                Err(CylinderWeightError::InvalidDensity) => {
                    p.push("density_kg_m3", "invalid", CylinderWeightError::InvalidDensity.to_string());
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

        let id = id.ok_or_else(|| ApplicationError::Infrastructure("id missing after validation".to_string()))?;
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
        let id = id.ok_or_else(|| ApplicationError::Infrastructure("id missing after validation".to_string()))?;

        match repo.delete(&id) {
            Ok(()) => Ok(()),
            Err(e) if e == "material_not_found" => Err(ApplicationError::Infrastructure(
                "material_not_found".to_string(),
            )),
            Err(e) => Err(ApplicationError::Infrastructure(e)),
        }
    }
}
