use serde::Serialize;

#[derive(Serialize)]
pub struct SolveCylinderWeightResponse {
    pub material_name: String,
    pub density_kg_m3: f64,
    pub outer_diameter_mm: f64,
    pub inner_diameter_mm: f64,
    pub length_mm: f64,
    pub mass_kg: f64,
}

#[derive(Serialize)]
pub struct CylinderMaterialResponse {
    pub id: String,
    pub name: String,
    pub density_kg_m3: f64,
}

#[derive(Serialize)]
pub struct ImportCylinderMaterialsResponse {
    pub imported: usize,
    pub skipped_duplicates: usize,
    pub skipped_invalid: usize,
    pub added: Vec<ImportAddedMaterialResponse>,
    pub skipped: Vec<ImportSkippedMaterialResponse>,
}

#[derive(Serialize)]
pub struct ImportAddedMaterialResponse {
    pub name: String,
    pub density_kg_m3: f64,
    pub original_name: Option<String>,
}

#[derive(Serialize)]
pub struct ImportSkippedMaterialResponse {
    pub name: Option<String>,
    pub density_kg_m3: Option<f64>,
    pub reason: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ExportCylinderMaterialResponse {
    pub name: String,
    pub density_kg_m3: f64,
}

#[derive(Serialize)]
pub struct ExportCylinderMaterialsResponse {
    pub schema_version: u32,
    pub materials: Vec<ExportCylinderMaterialResponse>,
}
