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
