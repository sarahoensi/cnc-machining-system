use super::CylinderWeightError;

#[derive(Debug, Copy, Clone)]
pub struct CylinderSpec {
    outer_diameter_mm: f64,
    inner_diameter_mm: f64,
    length_mm: f64,
}

impl CylinderSpec {
    pub fn new(
        outer_diameter_mm: f64,
        inner_diameter_mm: f64,
        length_mm: f64,
    ) -> Result<Self, CylinderWeightError> {
        if !outer_diameter_mm.is_finite() || outer_diameter_mm <= 0.0 {
            return Err(CylinderWeightError::InvalidOuterDiameter);
        }
        if !inner_diameter_mm.is_finite() || inner_diameter_mm < 0.0 {
            return Err(CylinderWeightError::InvalidInnerDiameter);
        }
        if !length_mm.is_finite() || length_mm <= 0.0 {
            return Err(CylinderWeightError::InvalidLength);
        }
        if inner_diameter_mm >= outer_diameter_mm {
            return Err(CylinderWeightError::InnerDiameterNotSmallerThanOuter);
        }

        Ok(Self {
            outer_diameter_mm,
            inner_diameter_mm,
            length_mm,
        })
    }

    pub fn outer_diameter_mm(&self) -> f64 {
        self.outer_diameter_mm
    }

    pub fn inner_diameter_mm(&self) -> f64 {
        self.inner_diameter_mm
    }

    pub fn length_mm(&self) -> f64 {
        self.length_mm
    }
}
