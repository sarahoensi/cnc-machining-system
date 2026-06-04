//! Mapping between right-triangle Tauri DTOs and application DTOs.
//!
//! Translates frontend request variants into application input and
//! application output into UI responses.

use crate::application::{SolveRightTriangleInput, SolveRightTriangleOutput};

use super::{SolveRightTriangleRequest, SolveRightTriangleResponse};

// ---------------------------------------------------------
// Request → Application Input
// ---------------------------------------------------------

impl From<SolveRightTriangleRequest> for SolveRightTriangleInput {
    fn from(req: SolveRightTriangleRequest) -> Self {
        match req {
            // -------------------------
            // SIDE + SIDE
            // -------------------------
            SolveRightTriangleRequest::Legs { a_mm, b_mm } => {
                SolveRightTriangleInput::Legs { a_mm, b_mm }
            }

            SolveRightTriangleRequest::LegAAndHypotenuse { a_mm, c_mm } => {
                SolveRightTriangleInput::LegAAndHypotenuse { a_mm, c_mm }
            }

            SolveRightTriangleRequest::LegBAndHypotenuse { b_mm, c_mm } => {
                SolveRightTriangleInput::LegBAndHypotenuse { b_mm, c_mm }
            }

            // -------------------------
            // SIDE + ANGLE
            // -------------------------
            SolveRightTriangleRequest::LegAAndAlpha { a_mm, alpha_deg } => {
                SolveRightTriangleInput::LegAAndAlpha { a_mm, alpha_deg }
            }

            SolveRightTriangleRequest::LegAAndBeta { a_mm, beta_deg } => {
                SolveRightTriangleInput::LegAAndBeta { a_mm, beta_deg }
            }

            SolveRightTriangleRequest::LegBAndAlpha { b_mm, alpha_deg } => {
                SolveRightTriangleInput::LegBAndAlpha { b_mm, alpha_deg }
            }

            SolveRightTriangleRequest::LegBAndBeta { b_mm, beta_deg } => {
                SolveRightTriangleInput::LegBAndBeta { b_mm, beta_deg }
            }

            SolveRightTriangleRequest::HypotenuseAndAlpha { c_mm, alpha_deg } => {
                SolveRightTriangleInput::HypotenuseAndAlpha { c_mm, alpha_deg }
            }

            SolveRightTriangleRequest::HypotenuseAndBeta { c_mm, beta_deg } => {
                SolveRightTriangleInput::HypotenuseAndBeta { c_mm, beta_deg }
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
