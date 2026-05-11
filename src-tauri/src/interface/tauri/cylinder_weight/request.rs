use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct SolveCylinderWeightRequest {
    pub material_id: Option<String>,
    pub outer_diameter_mm: Option<f64>,
    pub inner_diameter_mm: Option<f64>,
    pub length_mm: Option<f64>,
}

#[derive(Default, Deserialize)]
pub struct CreateCylinderMaterialRequest {
    pub name: Option<String>,
    pub density_kg_m3: Option<f64>,
}

#[derive(Default, Deserialize)]
pub struct UpdateCylinderMaterialRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub density_kg_m3: Option<f64>,
}

#[derive(Default, Deserialize)]
pub struct DeleteCylinderMaterialRequest {
    pub id: Option<String>,
}

#[derive(Default, Deserialize)]
pub struct ImportCylinderMaterialsRequest {
    pub json_payload: Option<String>,
}
