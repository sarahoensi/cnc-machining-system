//! Use case for right-triangle geometry orchestration.
//!
//! This module coordinates application input variants with domain triangle
//! solver services for machining geometry workflows.

use crate::application::right_triangle::dto::{
    SolveRightTriangleInput,
    SolveRightTriangleOutput,
};

use crate::application::shared::AppResult;
use crate::application::{ValidationErrors, ApplicationError};
use crate::domain::GeometryError;
use crate::application::shared::{
    validate_length_mm_positive,
    validate_angle_degrees,
    ensure_acute_angle,
};

use crate::domain::{
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

            // ==============================
            // a + b
            // ==============================
            SolveRightTriangleInput::Legs { a_mm, b_mm } => {

                let mut v = ValidationErrors::new();

                let a = validate_length_mm_positive("a", a_mm, &mut v);
                let b = validate_length_mm_positive("b", b_mm, &mut v);

                if !v.is_empty() {
                    return Err(ApplicationError::Validation(v));
                }

                match RightTriangleSolver::from_legs(a.unwrap(), b.unwrap()) {
                    Ok(t) => Ok(t),
                    Err(GeometryError::ImpossibleTriangle) => {
                        let mut v = ValidationErrors::new();
                        v.push("a", "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                        v.push("b", "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                        Err(ApplicationError::Validation(v))
                    }
                    Err(e) => Err(ApplicationError::Geometry(e)),
                }
            }

            // ==============================
            // a + alpha
            // ==============================
            SolveRightTriangleInput::LegAAndAlpha { a_mm, alpha_deg } => {

                let mut v = ValidationErrors::new();

                let a = validate_length_mm_positive("a", a_mm, &mut v);

                let alpha = validate_angle_degrees("alpha", alpha_deg, &mut v)
                    .and_then(|a| ensure_acute_angle("alpha", a, &mut v));

                if !v.is_empty() {
                    return Err(ApplicationError::Validation(v));
                }

                match RightTriangleSolver::from_leg_and_opposite_angle(
                    a.unwrap(),
                    alpha.unwrap(),
                ) {
                    Ok(t) => Ok(t),
                    Err(GeometryError::ImpossibleTriangle) => {
                        let mut v = ValidationErrors::new();
                        v.push("a", "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                        v.push("alpha", "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                        Err(ApplicationError::Validation(v))
                    }
                    Err(e) => Err(ApplicationError::Geometry(e)),
                }
            }

            // ==============================
            // a + c
            // ==============================
            SolveRightTriangleInput::LegAAndHypotenuse { a_mm, c_mm } => {

                let mut v = ValidationErrors::new();

                let a = validate_length_mm_positive("a", a_mm, &mut v);
                let c = validate_length_mm_positive("c", c_mm, &mut v);

                if !v.is_empty() {
                    return Err(ApplicationError::Validation(v));
                }

                match RightTriangleSolver::from_leg_and_hypotenuse(
                    a.unwrap(),
                    c.unwrap(),
                ) {
                    Ok(t) => Ok(t),
                    Err(GeometryError::ImpossibleTriangle) => {
                        let mut v = ValidationErrors::new();
                        v.push("a", "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                        v.push("c", "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                        Err(ApplicationError::Validation(v))
                    }
                    Err(e) => Err(ApplicationError::Geometry(e)),
                }
            }

            // ==============================
            // Resten følger samme mønster
            // ==============================

            SolveRightTriangleInput::LegBAndHypotenuse { b_mm, c_mm } => {
                self.solve_two_lengths(
                    "b", b_mm,
                    "c", c_mm,
                    |b, c| RightTriangleSolver::from_other_leg_and_hypotenuse(b, c),
                )
            }

            SolveRightTriangleInput::LegAAndBeta { a_mm, beta_deg } => {
                self.solve_length_and_angle(
                    "a", a_mm,
                    "beta", beta_deg,
                    |a, beta| RightTriangleSolver::from_leg_a_and_beta(a, beta),
                )
            }

            SolveRightTriangleInput::LegBAndAlpha { b_mm, alpha_deg } => {
                self.solve_length_and_angle(
                    "b", b_mm,
                    "alpha", alpha_deg,
                    |b, alpha| RightTriangleSolver::from_adjacent_leg_and_angle(b, alpha),
                )
            }

            SolveRightTriangleInput::LegBAndBeta { b_mm, beta_deg } => {
                self.solve_length_and_angle(
                    "b", b_mm,
                    "beta", beta_deg,
                    |b, beta| RightTriangleSolver::from_leg_b_and_beta(b, beta),
                )
            }

            SolveRightTriangleInput::HypotenuseAndAlpha { c_mm, alpha_deg } => {
                self.solve_length_and_angle(
                    "c", c_mm,
                    "alpha", alpha_deg,
                    |c, alpha| RightTriangleSolver::from_hypotenuse_and_angle(c, alpha),
                )
            }

            SolveRightTriangleInput::HypotenuseAndBeta { c_mm, beta_deg } => {
                self.solve_length_and_angle(
                    "c", c_mm,
                    "beta", beta_deg,
                    |c, beta| RightTriangleSolver::from_hypotenuse_and_beta(c, beta),
                )
            }
        }
    }

    // ============================================
    // Shared internal helpers for less repetition
    // ============================================

    fn solve_two_lengths<F>(
        &self,
        f1: &'static str,
        v1: f64,
        f2: &'static str,
        v2: f64,
        solver: F,
    ) -> AppResult<RightTriangle>
    where
        F: FnOnce(crate::domain::units::Length, crate::domain::units::Length)
            -> Result<RightTriangle, GeometryError>,
    {
        let mut v = ValidationErrors::new();

        let l1 = validate_length_mm_positive(f1, v1, &mut v);
        let l2 = validate_length_mm_positive(f2, v2, &mut v);

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        match solver(l1.unwrap(), l2.unwrap()) {
            Ok(t) => Ok(t),
            Err(GeometryError::ImpossibleTriangle) => {
                let mut v = ValidationErrors::new();
                v.push(f1, "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                v.push(f2, "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                Err(ApplicationError::Validation(v))
            }
            Err(e) => Err(ApplicationError::Geometry(e)),
        }
    }

    fn solve_length_and_angle<F>(
        &self,
        length_field: &'static str,
        length_raw: f64,
        angle_field: &'static str,
        angle_raw: f64,
        solver: F,
    ) -> AppResult<RightTriangle>
    where
        F: FnOnce(crate::domain::units::Length, crate::domain::units::Angle)
            -> Result<RightTriangle, GeometryError>,
    {
        let mut v = ValidationErrors::new();

        let length = validate_length_mm_positive(length_field, length_raw, &mut v);

        let angle = validate_angle_degrees(angle_field, angle_raw, &mut v)
            .and_then(|a| ensure_acute_angle(angle_field, a, &mut v));

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        match solver(length.unwrap(), angle.unwrap()) {
            Ok(t) => Ok(t),
            Err(GeometryError::ImpossibleTriangle) => {
                let mut v = ValidationErrors::new();
                v.push(length_field, "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                v.push(angle_field, "impossible_triangle", "Kan ikke danne rettvinklet trekant");
                Err(ApplicationError::Validation(v))
            }
            Err(e) => Err(ApplicationError::Geometry(e)),
        }
    }
}