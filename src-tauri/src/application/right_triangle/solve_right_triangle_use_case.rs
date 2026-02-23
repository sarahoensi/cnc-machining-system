//! Use case for right-triangle geometry orchestration (Variant A).
//!
//! - Application parses raw inputs into domain value objects (collecting field errors).
//! - Domain enforces triangle rules.
//! - Application maps GeometryError back into field-level ValidationErrors.

use crate::application::right_triangle::dto::{
    SolveRightTriangleInput,
    SolveRightTriangleOutput,
};

use crate::application::shared::AppResult;
use crate::application::{ApplicationError, ValidationErrors};

use crate::domain::{
    GeometryError,
    RightTriangle,
    RightTriangleSolver,
};

// Import value objects directly (no shared validation helpers)
use crate::domain::units::{Angle, Length};

// Needed for granular mapping (adjust path if you re-export it)
use crate::domain::RightTriangleError;

pub struct SolveRightTriangleUseCase;

impl SolveRightTriangleUseCase {
    pub fn execute(
        &self,
        input: SolveRightTriangleInput,
    ) -> AppResult<SolveRightTriangleOutput> {
        let triangle = self.solve_triangle(input)?;
        Ok(triangle.into())
    }

    fn solve_triangle(
        &self,
        input: SolveRightTriangleInput,
    ) -> AppResult<RightTriangle> {
        match input {
            // a + b
            SolveRightTriangleInput::Legs { a_mm, b_mm } => self.solve_two_lengths(
                "a",
                a_mm,
                "b",
                b_mm,
                RightTriangleSolver::from_legs,
            ),

            // a + c
            SolveRightTriangleInput::LegAAndHypotenuse { a_mm, c_mm } => self.solve_two_lengths(
                "a",
                a_mm,
                "c",
                c_mm,
                RightTriangleSolver::from_leg_and_hypotenuse,
            ),

            // b + c
            SolveRightTriangleInput::LegBAndHypotenuse { b_mm, c_mm } => self.solve_two_lengths(
                "b",
                b_mm,
                "c",
                c_mm,
                RightTriangleSolver::from_other_leg_and_hypotenuse,
            ),

            // a + alpha
            SolveRightTriangleInput::LegAAndAlpha { a_mm, alpha_deg } => self.solve_length_and_angle(
                "a",
                a_mm,
                "alpha",
                alpha_deg,
                RightTriangleSolver::from_leg_and_opposite_angle,
            ),

            // a + beta
            SolveRightTriangleInput::LegAAndBeta { a_mm, beta_deg } => self.solve_length_and_angle(
                "a",
                a_mm,
                "beta",
                beta_deg,
                RightTriangleSolver::from_leg_a_and_beta,
            ),

            // b + alpha
            SolveRightTriangleInput::LegBAndAlpha { b_mm, alpha_deg } => self.solve_length_and_angle(
                "b",
                b_mm,
                "alpha",
                alpha_deg,
                RightTriangleSolver::from_adjacent_leg_and_angle,
            ),

            // b + beta
            SolveRightTriangleInput::LegBAndBeta { b_mm, beta_deg } => self.solve_length_and_angle(
                "b",
                b_mm,
                "beta",
                beta_deg,
                RightTriangleSolver::from_leg_b_and_beta,
            ),

            // c + alpha
            SolveRightTriangleInput::HypotenuseAndAlpha { c_mm, alpha_deg } => self.solve_length_and_angle(
                "c",
                c_mm,
                "alpha",
                alpha_deg,
                RightTriangleSolver::from_hypotenuse_and_angle,
            ),

            // c + beta
            SolveRightTriangleInput::HypotenuseAndBeta { c_mm, beta_deg } => self.solve_length_and_angle(
                "c",
                c_mm,
                "beta",
                beta_deg,
                RightTriangleSolver::from_hypotenuse_and_beta,
            ),
        }
    }

    // ---------------------------------------------------------
    // Local parsing helpers (no shared validation module)
    // ---------------------------------------------------------

    fn parse_length_positive(
        field: &'static str,
        raw_mm: f64,
        v: &mut ValidationErrors,
    ) -> Option<Length> {
        match Length::mm_positive(raw_mm) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(field, "invalid", e.to_string());
                None
            }
        }
    }

    fn parse_angle_degrees(
        field: &'static str,
        raw_deg: f64,
        v: &mut ValidationErrors,
    ) -> Option<Angle> {
        match Angle::degrees(raw_deg) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(field, "invalid", e.to_string());
                None
            }
        }
    }

    // ---------------------------------------------------------
    // Shared solve helpers
    // ---------------------------------------------------------

    fn solve_two_lengths<F>(
        &self,
        field1: &'static str,
        raw1: f64,
        field2: &'static str,
        raw2: f64,
        solver: F,
    ) -> AppResult<RightTriangle>
    where
        F: FnOnce(Length, Length) -> Result<RightTriangle, GeometryError>,
    {
        let mut v = ValidationErrors::new();

        let l1 = Self::parse_length_positive(field1, raw1, &mut v);
        let l2 = Self::parse_length_positive(field2, raw2, &mut v);

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        solver(l1.unwrap(), l2.unwrap())
            .map_err(|e| map_triangle_error_two_fields(e, field1, field2))
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
        F: FnOnce(Length, Angle) -> Result<RightTriangle, GeometryError>,
    {
        let mut v = ValidationErrors::new();

        let length = Self::parse_length_positive(length_field, length_raw, &mut v);
        let angle = Self::parse_angle_degrees(angle_field, angle_raw, &mut v);

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        solver(length.unwrap(), angle.unwrap())
            .map_err(|e| map_triangle_error_length_and_angle(e, length_field, angle_field))
    }
}

// ---------------------------------------------------------
// GeometryError -> ValidationErrors mapping (inline-friendly)
// ---------------------------------------------------------

fn map_triangle_error_two_fields(
    err: GeometryError,
    f1: &'static str,
    f2: &'static str,
) -> ApplicationError {
    let mut v = ValidationErrors::new();

    match err {
        GeometryError::RightTriangle(rt) => match rt {
            RightTriangleError::HypotenuseTooShort { .. } => {
                let msg = rt.to_string();
                v.push(f1, "invalid_combination", &msg);
                v.push(f2, "invalid_combination", msg);
            }

            // Generic “triangle” error if it’s not clearly attributable
            other => {
                v.push("triangle", "impossible_triangle", other.to_string());
            }
        },

        other => {
            v.push("triangle", "impossible_triangle", other.to_string());
        }
    }

    ApplicationError::Validation(v)
}

fn map_triangle_error_length_and_angle(
    err: GeometryError,
    length_field: &'static str,
    angle_field: &'static str,
) -> ApplicationError {
    let mut v = ValidationErrors::new();

    match err {
        GeometryError::RightTriangle(rt) => match rt {
            // This is the domain rule you removed from app-level validation:
            RightTriangleError::AngleNotAcute { .. } => {
                v.push(angle_field, "out_of_range", rt.to_string());
            }

            // Some errors are best shown on both fields as “invalid combination”
            RightTriangleError::DivisionByZero => {
                let msg = rt.to_string();
                v.push(length_field, "invalid_combination", &msg);
                v.push(angle_field, "invalid_combination", msg);
            }

            other => {
                v.push("triangle", "impossible_triangle", other.to_string());
            }
        },

        other => {
            v.push("triangle", "impossible_triangle", other.to_string());
        }
    }

    ApplicationError::Validation(v)
}