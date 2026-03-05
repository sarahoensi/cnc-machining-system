// application/right_triangle/solve_right_triangle_use_case.rs
use crate::application::{
    shared::{AppResult, InputParser},
    right_triangle::dto::{SolveRightTriangleInput, SolveRightTriangleOutput},
};

use crate::domain::{
    RightTriangle,
    units::{PositiveLength, AcuteAngle},
};

pub struct SolveRightTriangleUseCase;

impl SolveRightTriangleUseCase {

    pub fn execute(
        &self,
        input: SolveRightTriangleInput,
    ) -> AppResult<SolveRightTriangleOutput> {

        let mut p = InputParser::new();

        let triangle = match input {

            SolveRightTriangleInput::Legs { a_mm, b_mm } => {

                let a = p.value("a", PositiveLength::mm(a_mm));
                let b = p.value("b", PositiveLength::mm(b_mm));

                p.map2(a, b, RightTriangle::from_legs)
            }

            SolveRightTriangleInput::LegAAndHypotenuse { a_mm, c_mm } => {

                let a = p.value("a", PositiveLength::mm(a_mm));
                let c = p.value("c", PositiveLength::mm(c_mm));

                p.combine("c", a, c, RightTriangle::from_leg_and_hypotenuse)
            }

            SolveRightTriangleInput::LegBAndHypotenuse { b_mm, c_mm } => {

                let b = p.value("b", PositiveLength::mm(b_mm));
                let c = p.value("c", PositiveLength::mm(c_mm));

                p.combine("c", b, c, RightTriangle::from_other_leg_and_hypotenuse)
            }

            SolveRightTriangleInput::HypotenuseAndAlpha { c_mm, alpha_deg } => {

                let c = p.value("c", PositiveLength::mm(c_mm));
                let alpha = p.value("alpha", AcuteAngle::degrees(alpha_deg));

                p.map2(c, alpha, RightTriangle::from_hypotenuse_and_angle)
            }

            SolveRightTriangleInput::HypotenuseAndBeta { c_mm, beta_deg } => {

                let c = p.value("c", PositiveLength::mm(c_mm));
                let beta = p.value("beta", AcuteAngle::degrees(beta_deg));

                p.map2(c, beta, RightTriangle::from_hypotenuse_and_beta)
            }

            SolveRightTriangleInput::LegAAndAlpha { a_mm, alpha_deg } => {

                let a = p.value("a", PositiveLength::mm(a_mm));
                let alpha = p.value("alpha", AcuteAngle::degrees(alpha_deg));

                p.combine("alpha", a, alpha, RightTriangle::from_leg_and_opposite_angle)
            }

            SolveRightTriangleInput::LegAAndBeta { a_mm, beta_deg } => {

                let a = p.value("a", PositiveLength::mm(a_mm));
                let beta = p.value("beta", AcuteAngle::degrees(beta_deg));

                p.combine("beta", a, beta, RightTriangle::from_leg_a_and_beta)
            }

            SolveRightTriangleInput::LegBAndAlpha { b_mm, alpha_deg } => {

                let b = p.value("b", PositiveLength::mm(b_mm));
                let alpha = p.value("alpha", AcuteAngle::degrees(alpha_deg));

                p.combine("alpha", b, alpha, RightTriangle::from_adjacent_leg_and_angle)
            }

            SolveRightTriangleInput::LegBAndBeta { b_mm, beta_deg } => {

                let b = p.value("b", PositiveLength::mm(b_mm));
                let beta = p.value("beta", AcuteAngle::degrees(beta_deg));

                p.combine("beta", b, beta, RightTriangle::from_leg_b_and_beta)
            }
        };

        let triangle = p.finish_with(triangle)?;

        Ok(triangle.into())
    }
}