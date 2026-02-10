// application/right_triangle/dto.rs

use crate::domain::RightTriangle;

pub enum SolveRightTriangleInput {
    Legs {
        a_mm: f64,
        b_mm: f64,
    },
    LegAndHypotenuse {
        a_mm: f64,
        c_mm: f64,
    },
    OtherLegAndHypotenuse {
        b_mm: f64,
        c_mm: f64,
    },
    HypotenuseAndAngle {
        c_mm: f64,
        alpha_deg: f64,
    },
}

pub struct SolveRightTriangleOutput {
    pub a_mm: f64,
    pub b_mm: f64,
    pub c_mm: f64,
    pub alpha_deg: f64,
    pub beta_deg: f64,
}

// ---------------------------------------------------------
// Domain → Application DTO mapping
// ---------------------------------------------------------

impl From<RightTriangle> for SolveRightTriangleOutput {
    fn from(triangle: RightTriangle) -> Self {
        Self {
            a_mm: triangle.a().mm_value(),
            b_mm: triangle.b().mm_value(),
            c_mm: triangle.c().mm_value(),
            alpha_deg: triangle.alpha().degrees_value(),
            beta_deg: triangle.beta().degrees_value(),
        }
    }
}
