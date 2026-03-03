//! Use case for right-triangle orchestration (fully aligned with domain)

use crate::application::shared::AppResult;
use crate::application::{ApplicationError, ValidationErrors};

use crate::application::right_triangle::dto::{
    SolveRightTriangleInput,
    SolveRightTriangleOutput,
};

use crate::domain::{
    GeometryError,
    RightTriangle,
    RightTriangleSolver,
    RightTriangleError,
    units::{PositiveLength, AcuteAngle},
};

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

        SolveRightTriangleInput::Legs { a_mm, b_mm } => {
            let (a, b) = self.parse_two_lengths("a", a_mm, "b", b_mm)?;
            RightTriangleSolver::from_legs(a, b)
                .map_err(|e| map_triangle_error_two_fields(e, "a", "b"))
        }

        SolveRightTriangleInput::LegAAndHypotenuse { a_mm, c_mm } => {
            let (a, c) = self.parse_two_lengths("a", a_mm, "c", c_mm)?;
            RightTriangleSolver::from_leg_and_hypotenuse(a, c)
                .map_err(|e| map_triangle_error_two_fields(e, "a", "c"))
        }

        SolveRightTriangleInput::LegBAndHypotenuse { b_mm, c_mm } => {
            let (b, c) = self.parse_two_lengths("b", b_mm, "c", c_mm)?;
            RightTriangleSolver::from_other_leg_and_hypotenuse(b, c)
                .map_err(|e| map_triangle_error_two_fields(e, "b", "c"))
        }

        SolveRightTriangleInput::HypotenuseAndAlpha { c_mm, alpha_deg } => {
            let c = self.parse_length("c", c_mm)?;
            let alpha = self.parse_angle("alpha", alpha_deg)?;
            RightTriangleSolver::from_hypotenuse_and_angle(c, alpha)
                .map_err(|e| map_triangle_error_length_and_angle(e, "c", "alpha"))
        }

        SolveRightTriangleInput::HypotenuseAndBeta { c_mm, beta_deg } => {
            let c = self.parse_length("c", c_mm)?;
            let beta = self.parse_angle("beta", beta_deg)?;
            RightTriangleSolver::from_hypotenuse_and_beta(c, beta)
                .map_err(|e| map_triangle_error_length_and_angle(e, "c", "beta"))
        }

        SolveRightTriangleInput::LegAAndAlpha { a_mm, alpha_deg } => {
            let a = self.parse_length("a", a_mm)?;
            let alpha = self.parse_angle("alpha", alpha_deg)?;
            RightTriangleSolver::from_leg_and_opposite_angle(a, alpha)
                .map_err(|e| map_triangle_error_length_and_angle(e, "a", "alpha"))
        }

        SolveRightTriangleInput::LegAAndBeta { a_mm, beta_deg } => {
            let a = self.parse_length("a", a_mm)?;
            let beta = self.parse_angle("beta", beta_deg)?;
            RightTriangleSolver::from_leg_a_and_beta(a, beta)
                .map_err(|e| map_triangle_error_length_and_angle(e, "a", "beta"))
        }

        SolveRightTriangleInput::LegBAndAlpha { b_mm, alpha_deg } => {
            let b = self.parse_length("b", b_mm)?;
            let alpha = self.parse_angle("alpha", alpha_deg)?;
            RightTriangleSolver::from_adjacent_leg_and_angle(b, alpha)
                .map_err(|e| map_triangle_error_length_and_angle(e, "b", "alpha"))
        }

        SolveRightTriangleInput::LegBAndBeta { b_mm, beta_deg } => {
            let b = self.parse_length("b", b_mm)?;
            let beta = self.parse_angle("beta", beta_deg)?;
            RightTriangleSolver::from_leg_b_and_beta(b, beta)
                .map_err(|e| map_triangle_error_length_and_angle(e, "b", "beta"))
        }
    }
}

    // ---------------------------------------------------------
    // Parsing helpers (Helix-style)
    // ---------------------------------------------------------

    fn parse_two_lengths(
        &self,
        f1: &'static str,
        raw1: f64,
        f2: &'static str,
        raw2: f64,
    ) -> AppResult<(PositiveLength, PositiveLength)> {

        let mut v = ValidationErrors::new();

        let l1 = match PositiveLength::mm(raw1) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(f1, "invalid", e.to_string());
                None
            }
        };

        let l2 = match PositiveLength::mm(raw2) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(f2, "invalid", e.to_string());
                None
            }
        };

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        Ok((l1.unwrap(), l2.unwrap()))
    }

    fn parse_length(
        &self,
        field: &'static str,
        raw: f64,
    ) -> AppResult<PositiveLength> {

        PositiveLength::mm(raw)
            .map_err(|e| single_field_error(field, "invalid", e.to_string()))
    }

    fn parse_angle(
        &self,
        field: &'static str,
        raw: f64,
    ) -> AppResult<AcuteAngle> {

        AcuteAngle::degrees(raw)
            .map_err(|e| single_field_error(field, "out_of_range", e.to_string()))
    }
}

// ---------------------------------------------------------
// GeometryError -> ValidationErrors mapping
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

            RightTriangleError::LegNotPositive { .. } => {
                v.push(f1, "invalid", rt.to_string());
            }

            other => {
                v.push("triangle", "invalid_geometry", other.to_string());
            }
        },

        other => {
            v.push("triangle", "invalid_geometry", other.to_string());
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

            RightTriangleError::DivisionByZero => {
                let msg = rt.to_string();
                v.push(length_field, "invalid_combination", &msg);
                v.push(angle_field, "invalid_combination", msg);
            }

            RightTriangleError::NumericalInstability => {
                v.push("triangle", "numerical_instability", rt.to_string());
            }

            other => {
                v.push("triangle", "invalid_geometry", other.to_string());
            }
        },

        other => {
            v.push("triangle", "invalid_geometry", other.to_string());
        }
    }

    ApplicationError::Validation(v)
}

fn single_field_error(
    field: &'static str,
    code: &'static str,
    message: String,
) -> ApplicationError {

    let mut v = ValidationErrors::new();
    v.push(field, code, message);
    ApplicationError::Validation(v)
}