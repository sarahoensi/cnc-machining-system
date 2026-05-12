use crate::application::{
    CreateCylinderMaterialInput, CylinderMaterialOutput, DeleteCylinderMaterialInput,
    ExportCylinderMaterialRow, ExportCylinderMaterialsOutput, ImportCylinderMaterialsInput,
    ImportAddedMaterialRow, ImportCylinderMaterialsOutput, ImportSkippedMaterialRow,
    SolveCylinderWeightInput,
    SolveCylinderWeightOutput,
    UpdateCylinderMaterialInput,
};

use super::{
    CreateCylinderMaterialRequest, CylinderMaterialResponse, DeleteCylinderMaterialRequest,
    ExportCylinderMaterialResponse, ExportCylinderMaterialsResponse, ImportCylinderMaterialsRequest,
    ImportAddedMaterialResponse, ImportCylinderMaterialsResponse, ImportSkippedMaterialResponse,
    SolveCylinderWeightRequest,
    SolveCylinderWeightResponse,
    UpdateCylinderMaterialRequest,
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

impl From<UpdateCylinderMaterialRequest> for UpdateCylinderMaterialInput {
    fn from(req: UpdateCylinderMaterialRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            density_kg_m3: req.density_kg_m3,
        }
    }
}

impl From<DeleteCylinderMaterialRequest> for DeleteCylinderMaterialInput {
    fn from(req: DeleteCylinderMaterialRequest) -> Self {
        Self { id: req.id }
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

impl From<ImportCylinderMaterialsRequest> for ImportCylinderMaterialsInput {
    fn from(req: ImportCylinderMaterialsRequest) -> Self {
        Self {
            json_payload: req.json_payload,
        }
    }
}

impl From<ImportCylinderMaterialsOutput> for ImportCylinderMaterialsResponse {
    fn from(out: ImportCylinderMaterialsOutput) -> Self {
        Self {
            imported: out.imported,
            skipped_duplicates: out.skipped_duplicates,
            skipped_invalid: out.skipped_invalid,
            added: out.added.into_iter().map(Into::into).collect(),
            skipped: out.skipped.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ImportAddedMaterialRow> for ImportAddedMaterialResponse {
    fn from(out: ImportAddedMaterialRow) -> Self {
        Self {
            name: out.name,
            density_kg_m3: out.density_kg_m3,
            original_name: out.original_name,
        }
    }
}

impl From<ImportSkippedMaterialRow> for ImportSkippedMaterialResponse {
    fn from(out: ImportSkippedMaterialRow) -> Self {
        Self {
            name: out.name,
            density_kg_m3: out.density_kg_m3,
            reason: out.reason,
            message: out.message,
        }
    }
}

impl From<ExportCylinderMaterialRow> for ExportCylinderMaterialResponse {
    fn from(out: ExportCylinderMaterialRow) -> Self {
        Self {
            name: out.name,
            density_kg_m3: out.density_kg_m3,
        }
    }
}

impl From<ExportCylinderMaterialsOutput> for ExportCylinderMaterialsResponse {
    fn from(out: ExportCylinderMaterialsOutput) -> Self {
        Self {
            schema_version: out.schema_version,
            materials: out.materials.into_iter().map(Into::into).collect(),
        }
    }
}
