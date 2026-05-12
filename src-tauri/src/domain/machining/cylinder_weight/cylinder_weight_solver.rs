use std::f64::consts::PI;

use super::{CylinderSpec, CylinderWeightError, Material};

pub struct CylinderWeightSolver;

impl CylinderWeightSolver {
    pub fn calculate_mass_kg(
        spec: CylinderSpec,
        material: &Material,
    ) -> Result<f64, CylinderWeightError> {
        let do_mm = spec.outer_diameter_mm();
        let di_mm = spec.inner_diameter_mm();
        let length_mm = spec.length_mm();

        let volume_mm3 = PI * length_mm * (do_mm * do_mm - di_mm * di_mm) / 4.0;
        if !volume_mm3.is_finite() || volume_mm3 <= 0.0 {
            return Err(CylinderWeightError::NumericalInstability);
        }

        let volume_m3 = volume_mm3 / 1_000_000_000.0;
        let mass_kg = material.density_kg_m3() * volume_m3;

        if !mass_kg.is_finite() || mass_kg <= 0.0 {
            return Err(CylinderWeightError::NumericalInstability);
        }

        Ok(mass_kg)
    }
}
