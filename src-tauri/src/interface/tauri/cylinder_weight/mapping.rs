use crate::application::{
    CreateCylinderMaterialInput, CylinderMaterialOutput, SolveCylinderWeightInput,
    SolveCylinderWeightOutput,
};

use super::{
    CreateCylinderMaterialRequest, CylinderMaterialResponse, SolveCylinderWeightRequest,
    SolveCylinderWeightResponse,
};

impl From<SolveCylinderWeightRequest> for SolveCylinderWeightInput {
    fn from(req: SolveCylinderWeightRequest) -> Self {
        Self {
            material_id: req.material_id,
            outer_diameter_mm: req.outer_diameter_mm,
            inner_diameter_mm: req.inner_diameter_mm,
            length_mm: req.length_mm,
        }
    }
}

impl From<SolveCylinderWeightOutput> for SolveCylinderWeightResponse {
    fn from(out: SolveCylinderWeightOutput) -> Self {
        Self {
            material_name: out.material_name,
            density_kg_m3: out.density_kg_m3,
            outer_diameter_mm: out.outer_diameter_mm,
            inner_diameter_mm: out.inner_diameter_mm,
            length_mm: out.length_mm,
            mass_kg: out.mass_kg,
        }
    }
}

impl From<CreateCylinderMaterialRequest> for CreateCylinderMaterialInput {
    fn from(req: CreateCylinderMaterialRequest) -> Self {
        Self {
            name: req.name,
            density_kg_m3: req.density_kg_m3,
        }
    }
}

impl From<CylinderMaterialOutput> for CylinderMaterialResponse {
    fn from(out: CylinderMaterialOutput) -> Self {
        Self {
            id: out.id,
            name: out.name,
            density_kg_m3: out.density_kg_m3,
        }
    }
}
