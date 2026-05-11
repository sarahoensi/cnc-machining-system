use crate::{
    application::{
        shared::{AppResult, InputParser},
        ApplicationError,
    },
    domain::machining::{CylinderSpec, CylinderWeightSolver, Material},
};

use super::{
    CreateCylinderMaterialInput, CylinderMaterialOutput, CylinderMaterialRepository,
    SolveCylinderWeightInput, SolveCylinderWeightOutput,
};

pub struct ListCylinderMaterialsUseCase;
pub struct CreateCylinderMaterialUseCase;
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
            (Some(n), Some(d)) => p.value("name", Material::new(n, d)),
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
        let inner = match input.inner_diameter_mm {
            Some(v) => Some(v),
            None => {
                p.push("inner_diameter_mm", "validation_error", "is required");
                None
            }
        };
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
            (Some(o), Some(i), Some(l)) => p.domain("inner_diameter_mm", CylinderSpec::new(o, i, l)),
            _ => None,
        };

        p.finish()?;

        let (_, material) = material.expect("material should exist after validation");
        let spec = spec.expect("spec should exist after validation");

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
