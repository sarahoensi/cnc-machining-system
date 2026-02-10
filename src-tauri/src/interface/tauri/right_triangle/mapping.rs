// interfaces/tauri/right_triangle/mapping.rs

use crate::application::{
    SolveRightTriangleInput,
    SolveRightTriangleOutput,
};

use super::{
    SolveRightTriangleRequest,
    SolveRightTriangleResponse,
};

// ---------------------------------------------------------
// Request → Application Input
// ---------------------------------------------------------

impl From<SolveRightTriangleRequest> for SolveRightTriangleInput {
    fn from(req: SolveRightTriangleRequest) -> Self {
        match req {

            SolveRightTriangleRequest::Legs { a_mm, b_mm } => {
                SolveRightTriangleInput::Legs { a_mm, b_mm }
            }

            SolveRightTriangleRequest::LegAndHypotenuse { a_mm, c_mm } => {
                SolveRightTriangleInput::LegAndHypotenuse { a_mm, c_mm }
            }

            SolveRightTriangleRequest::OtherLegAndHypotenuse { b_mm, c_mm } => {
                SolveRightTriangleInput::OtherLegAndHypotenuse { b_mm, c_mm }
            }

            SolveRightTriangleRequest::HypotenuseAndAngle { c_mm, alpha_deg } => {
                SolveRightTriangleInput::HypotenuseAndAngle {
                    c_mm,
                    alpha_deg,
                }
            }
        }
    }
}

// ---------------------------------------------------------
// Application Output → Response DTO
// ---------------------------------------------------------

impl From<SolveRightTriangleOutput> for SolveRightTriangleResponse {
    fn from(out: SolveRightTriangleOutput) -> Self {
        Self {
            a_mm: out.a_mm,
            b_mm: out.b_mm,
            c_mm: out.c_mm,
            alpha_deg: out.alpha_deg,
            beta_deg: out.beta_deg,
        }
    }
}
