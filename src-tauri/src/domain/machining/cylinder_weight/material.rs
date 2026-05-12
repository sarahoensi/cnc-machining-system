use super::CylinderWeightError;

#[derive(Debug, Clone)]
pub struct Material {
    name: String,
    normalized_name: String,
    density_kg_m3: f64,
}

impl Material {
    pub fn new(name: String, density_kg_m3: f64) -> Result<Self, CylinderWeightError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(CylinderWeightError::InvalidMaterialName);
        }
        if !density_kg_m3.is_finite() || density_kg_m3 <= 0.0 {
            return Err(CylinderWeightError::InvalidDensity);
        }

        Ok(Self {
            name: trimmed.to_string(),
            normalized_name: Self::normalize_name(trimmed),
            density_kg_m3,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn normalized_name(&self) -> &str {
        &self.normalized_name
    }

    pub fn density_kg_m3(&self) -> f64 {
        self.density_kg_m3
    }

    pub fn normalize_name(name: &str) -> String {
        name.trim().to_ascii_lowercase()
    }
}
