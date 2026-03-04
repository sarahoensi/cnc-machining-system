// application/right_triangle/solve_right_triangle_use_case.rs
use crate::application::shared::AppResult;

use crate::application::right_triangle::dto::{
    SolveRightTriangleInput,
    SolveRightTriangleOutput,
};

use crate::domain::{
    DomainError,
    RightTriangle,
    units::{PositiveLength, AcuteAngle},
};

pub struct SolveRightTriangleUseCase;

impl SolveRightTriangleUseCase {

    // ---------------------------------------------------------
    // Public entrypoint (Application boundary)
    // ---------------------------------------------------------

    pub fn execute(
        &self,
        input: SolveRightTriangleInput,
    ) -> AppResult<SolveRightTriangleOutput> {

        let triangle = self.solve_triangle(input)?;
        Ok(triangle.into())
    }

    // ---------------------------------------------------------
    // Internal orchestration (Domain boundary)
    // ---------------------------------------------------------

    fn solve_triangle(
        &self,
        input: SolveRightTriangleInput,
    ) -> Result<RightTriangle, DomainError> {

        match input {

            SolveRightTriangleInput::Legs { a_mm, b_mm } => {
                let a = PositiveLength::mm(a_mm)?;
                let b = PositiveLength::mm(b_mm)?;
                Ok(RightTriangle::from_legs(a, b))
            }

            SolveRightTriangleInput::LegAAndHypotenuse { a_mm, c_mm } => {
                let a = PositiveLength::mm(a_mm)?;
                let c = PositiveLength::mm(c_mm)?;
                Ok(RightTriangle::from_leg_and_hypotenuse(a, c)?)
            }

            SolveRightTriangleInput::LegBAndHypotenuse { b_mm, c_mm } => {
                let b = PositiveLength::mm(b_mm)?;
                let c = PositiveLength::mm(c_mm)?;
                Ok(RightTriangle::from_other_leg_and_hypotenuse(b, c)?)
            }

            SolveRightTriangleInput::HypotenuseAndAlpha { c_mm, alpha_deg } => {
                let c = PositiveLength::mm(c_mm)?;
                let alpha = AcuteAngle::degrees(alpha_deg)?;
                Ok(RightTriangle::from_hypotenuse_and_angle(c, alpha))
            }

            SolveRightTriangleInput::HypotenuseAndBeta { c_mm, beta_deg } => {
                let c = PositiveLength::mm(c_mm)?;
                let beta = AcuteAngle::degrees(beta_deg)?;
                Ok(RightTriangle::from_hypotenuse_and_beta(c, beta))
            }

            SolveRightTriangleInput::LegAAndAlpha { a_mm, alpha_deg } => {
                let a = PositiveLength::mm(a_mm)?;
                let alpha = AcuteAngle::degrees(alpha_deg)?;
                Ok(RightTriangle::from_leg_and_opposite_angle(a, alpha)?)
            }

            SolveRightTriangleInput::LegAAndBeta { a_mm, beta_deg } => {
                let a = PositiveLength::mm(a_mm)?;
                let beta = AcuteAngle::degrees(beta_deg)?;
                Ok(RightTriangle::from_leg_a_and_beta(a, beta)?)
            }

            SolveRightTriangleInput::LegBAndAlpha { b_mm, alpha_deg } => {
                let b = PositiveLength::mm(b_mm)?;
                let alpha = AcuteAngle::degrees(alpha_deg)?;
                Ok(RightTriangle::from_adjacent_leg_and_angle(b, alpha)?)
            }

            SolveRightTriangleInput::LegBAndBeta { b_mm, beta_deg } => {
                let b = PositiveLength::mm(b_mm)?;
                let beta = AcuteAngle::degrees(beta_deg)?;
                Ok(RightTriangle::from_leg_b_and_beta(b, beta)?)
            }
        }
    }
}