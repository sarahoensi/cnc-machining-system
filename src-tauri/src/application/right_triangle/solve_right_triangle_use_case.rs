//! Use case for right-triangle geometry orchestration.
//!
//! This module coordinates application input variants with domain triangle
//! solver services for machining geometry workflows.

use crate::application::right_triangle::dto::{
    SolveRightTriangleInput,
    SolveRightTriangleOutput,
};

use crate::application::shared::AppResult;

use crate::domain::{
    units::{Angle, Length},
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

            // -------------------------------------------------
            // SIDE + SIDE
            // -------------------------------------------------

            SolveRightTriangleInput::Legs { a_mm, b_mm } => {
                let a = Length::mm_positive(a_mm)?;
                let b = Length::mm_positive(b_mm)?;
                RightTriangleSolver::from_legs(a, b).map_err(Into::into)
            }

            SolveRightTriangleInput::LegAAndHypotenuse { a_mm, c_mm } => {
                let a = Length::mm_positive(a_mm)?;
                let c = Length::mm_positive(c_mm)?;
                RightTriangleSolver::from_leg_and_hypotenuse(a, c)
                    .map_err(Into::into)
            }

            SolveRightTriangleInput::LegBAndHypotenuse { b_mm, c_mm } => {
                let b = Length::mm_positive(b_mm)?;
                let c = Length::mm_positive(c_mm)?;
                RightTriangleSolver::from_other_leg_and_hypotenuse(b, c)
                    .map_err(Into::into)
            }

            // -------------------------------------------------
            // SIDE + ANGLE
            // -------------------------------------------------

            // a + alpha (alpha opposite a)
            SolveRightTriangleInput::LegAAndAlpha { a_mm, alpha_deg } => {
                let a = Length::mm_positive(a_mm)?;
                let alpha = Angle::degrees(alpha_deg)?;
                RightTriangleSolver::from_leg_and_opposite_angle(a, alpha)
                    .map_err(Into::into)
            }

            // a + beta
            SolveRightTriangleInput::LegAAndBeta { a_mm, beta_deg } => {
                let a = Length::mm_positive(a_mm)?;
                let beta = Angle::degrees(beta_deg)?;
                RightTriangleSolver::from_leg_a_and_beta(a, beta)
                    .map_err(Into::into)
            }

            // b + alpha (alpha adjacent to b)
            SolveRightTriangleInput::LegBAndAlpha { b_mm, alpha_deg } => {
                let b = Length::mm_positive(b_mm)?;
                let alpha = Angle::degrees(alpha_deg)?;
                RightTriangleSolver::from_adjacent_leg_and_angle(b, alpha)
                    .map_err(Into::into)
            }

            // b + beta
            SolveRightTriangleInput::LegBAndBeta { b_mm, beta_deg } => {
                let b = Length::mm_positive(b_mm)?;
                let beta = Angle::degrees(beta_deg)?;
                RightTriangleSolver::from_leg_b_and_beta(b, beta)
                    .map_err(Into::into)
            }

            // c + alpha
            SolveRightTriangleInput::HypotenuseAndAlpha { c_mm, alpha_deg } => {
                let c = Length::mm_positive(c_mm)?;
                let alpha = Angle::degrees(alpha_deg)?;
                RightTriangleSolver::from_hypotenuse_and_angle(c, alpha)
                    .map_err(Into::into)
            }

            // c + beta
            SolveRightTriangleInput::HypotenuseAndBeta { c_mm, beta_deg } => {
                let c = Length::mm_positive(c_mm)?;
                let beta = Angle::degrees(beta_deg)?;
                RightTriangleSolver::from_hypotenuse_and_beta(c, beta)
                    .map_err(Into::into)
            }
        }
    }
}