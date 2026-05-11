#[derive(Default)]
pub struct SolveCylinderWeightInput {
    pub material_id: Option<String>,
    pub outer_diameter_mm: Option<f64>,
    pub inner_diameter_mm: Option<f64>,
    pub length_mm: Option<f64>,
}

pub struct SolveCylinderWeightOutput {
    pub material_name: String,
    pub density_kg_m3: f64,
    pub outer_diameter_mm: f64,
    pub inner_diameter_mm: f64,
    pub length_mm: f64,
    pub mass_kg: f64,
}

#[derive(Default)]
pub struct CreateCylinderMaterialInput {
    pub name: Option<String>,
    pub density_kg_m3: Option<f64>,
}

#[derive(Default)]
pub struct UpdateCylinderMaterialInput {
    pub id: Option<String>,
    pub name: Option<String>,
    pub density_kg_m3: Option<f64>,
}

#[derive(Default)]
pub struct DeleteCylinderMaterialInput {
    pub id: Option<String>,
}

pub struct CylinderMaterialOutput {
    pub id: String,
    pub name: String,
    pub density_kg_m3: f64,
}
