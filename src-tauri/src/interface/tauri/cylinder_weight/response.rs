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
