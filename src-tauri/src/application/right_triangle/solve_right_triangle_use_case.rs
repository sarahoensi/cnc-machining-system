//! Use case for right-triangle geometry orchestration.
//!
//! This module coordinates application input variants with domain triangle
//! solver services for machining geometry workflows.

// application/right_triangle/solve_right_triangle_use_case.rs

use crate::application::right_triangle::dto::{
    SolveRightTriangleInput,
    SolveRightTriangleOutput,
};

use crate::application::shared::AppResult;

use crate::domain::{
    Angle,
    Length,
    RightTriangle,
    RightTriangleSolver,
};

pub struct SolveRightTriangleUseCase;

impl SolveRightTriangleUseCase {

    /// Solves a right triangle from a supported known-value combination.
    ///
    /// Purpose:
    /// - Routes input variants to the appropriate domain solving path.
    /// - Returns a normalized triangle DTO for external layers.
    ///
    /// Required inputs:
    /// - A [`SolveRightTriangleInput`] variant with valid lengths (`mm`) and,
    ///   when applicable, angle (`deg`).
    ///
    /// Output meaning:
    /// - [`SolveRightTriangleOutput`] containing all sides and acute angles.
    ///
    /// Domain invariants enforced:
    /// - Positive-length and angle constraints are validated by domain value
    ///   objects.
    /// - Right-triangle consistency rules are enforced by the domain solver.
    ///
    /// Side effects:
    /// - None. This use case is computation-only and does not persist state.
    ///
    /// Error scenarios:
    /// - Invalid or non-physical geometry combinations rejected by domain APIs.
    pub fn execute(
        &self,
        input: SolveRightTriangleInput,
    ) -> AppResult<SolveRightTriangleOutput> {

        let triangle = self.solve_triangle(input)?;

        Ok(triangle.into())
    }

    // ---------------------------------------------------------
    // Internal workflow
    // ---------------------------------------------------------

    fn solve_triangle(
    &self,
    input: SolveRightTriangleInput,
) -> AppResult<RightTriangle> {

    match input {

        SolveRightTriangleInput::Legs { a_mm, b_mm } => {
            let a = Length::mm_positive(a_mm)?;
            let b = Length::mm_positive(b_mm)?;

            RightTriangleSolver::from_legs(a, b).map_err(Into::into)
        }

        SolveRightTriangleInput::LegAndHypotenuse { a_mm, c_mm } => {
            let a = Length::mm_positive(a_mm)?;
            let c = Length::mm_positive(c_mm)?;

            RightTriangleSolver::from_leg_and_hypotenuse(a, c).map_err(Into::into)
        }

        SolveRightTriangleInput::OtherLegAndHypotenuse { b_mm, c_mm } => {
            let b = Length::mm_positive(b_mm)?;
            let c = Length::mm_positive(c_mm)?;

            RightTriangleSolver::from_other_leg_and_hypotenuse(b, c)
                .map_err(Into::into)
        }

        SolveRightTriangleInput::HypotenuseAndAngle { c_mm, alpha_deg } => {
            let c = Length::mm_positive(c_mm)?;
            let alpha = Angle::degrees(alpha_deg)?;

            RightTriangleSolver::from_hypotenuse_and_angle(c, alpha)
                .map_err(Into::into)
        }
    }
}

}

// --------- TESTS ----
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solves_from_legs() {

        let use_case = SolveRightTriangleUseCase;

        let input = SolveRightTriangleInput::Legs {
            a_mm: 3.0,
            b_mm: 4.0,
        };

        let result = use_case.execute(input).unwrap();

        assert!((result.c_mm - 5.0).abs() < 1e-9);
    }
}
