//! Use case for helix parameter orchestration.

use std::f64::consts::PI;

use crate::application::shared::AppResult;
use crate::application::{ApplicationError, ValidationErrors};

use crate::application::helix::dto::{
    SolveHelixInput,
    SolveHelixOutput,
};

use crate::domain::{
    units::{AcuteAngle, Diameter, Pitch},
    EffectiveDiameter,
    Helix,
    HelixError,
    HelixMode,
};

pub struct SolveHelixUseCase;

impl SolveHelixUseCase {

    pub fn execute(
        &self,
        input: SolveHelixInput,
    ) -> AppResult<SolveHelixOutput> {
        let helix = self.solve_helix(input)?;
        Ok(helix.into())
    }

    fn solve_helix(
        &self,
        input: SolveHelixInput,
    ) -> AppResult<Helix> {

        match input {

            SolveHelixInput::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => self.solve_from_pitch(
                mode.into(),
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            ),

            SolveHelixInput::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => self.solve_from_angle(
                mode.into(),
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            ),
        }
    }

    // ---------------------------------------------------------
    // Variant: Pitch
    // ---------------------------------------------------------

    fn solve_from_pitch(
        &self,
        mode: HelixMode,
        diameter_raw: f64,
        tool_raw: f64,
        pitch_raw: f64,
    ) -> AppResult<Helix> {

        let mut v = ValidationErrors::new();

        let diameter =
            Self::parse_diameter("diameter", diameter_raw, &mut v);

        let tool =
            Self::parse_diameter("toolDiameter", tool_raw, &mut v);

        let pitch =
            Self::parse_pitch("pitch", pitch_raw, &mut v);

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        let (diameter, tool, pitch) =
            (diameter.unwrap(), tool.unwrap(), pitch.unwrap());

        let effective =
            EffectiveDiameter::new(mode, diameter, tool)
                .map_err(map_effective_diameter_error)?;

        Ok(Helix::new(
            effective.diameter(),
            pitch,
        ))
    }

    // ---------------------------------------------------------
    // Variant: Angle
    // ---------------------------------------------------------

    fn solve_from_angle(
        &self,
        mode: HelixMode,
        diameter_raw: f64,
        tool_raw: f64,
        angle_raw: f64,
    ) -> AppResult<Helix> {

        let mut v = ValidationErrors::new();

        let diameter =
            Self::parse_diameter("diameter", diameter_raw, &mut v);

        let tool =
            Self::parse_diameter("toolDiameter", tool_raw, &mut v);

        let angle =
            Self::parse_angle("angle", angle_raw, &mut v);

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        let (diameter, tool, angle) =
            (diameter.unwrap(), tool.unwrap(), angle.unwrap());

        let effective =
            EffectiveDiameter::new(mode, diameter, tool)
                .map_err(map_effective_diameter_error)?;

        // Geometry:
        // pitch = tan(angle) * circumference
        let circumference =
            PI * effective.diameter().mm_value();

        let pitch_value =
            angle.radians_value().tan() * circumference;

        let pitch =
            Pitch::mm_per_rev(pitch_value)
                .map_err(|e| single_field_error(
                    "angle",
                    "invalid_combination",
                    e.to_string(),
                ))?;

        Ok(Helix::new(
            effective.diameter(),
            pitch,
        ))
    }

    // ---------------------------------------------------------
    // Parsing helpers
    // ---------------------------------------------------------

    fn parse_diameter(
        field: &'static str,
        raw: f64,
        v: &mut ValidationErrors,
    ) -> Option<Diameter> {

        match Diameter::mm(raw) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(field, "invalid", e.to_string());
                None
            }
        }
    }

    fn parse_pitch(
        field: &'static str,
        raw: f64,
        v: &mut ValidationErrors,
    ) -> Option<Pitch> {

        match Pitch::mm_per_rev(raw) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(field, "invalid", e.to_string());
                None
            }
        }
    }

    fn parse_angle(
        field: &'static str,
        raw: f64,
        v: &mut ValidationErrors,
    ) -> Option<AcuteAngle> {

        match AcuteAngle::degrees(raw) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(field, "invalid", e.to_string());
                None
            }
        }
    }
}

// ---------------------------------------------------------
// Error mapping
// ---------------------------------------------------------

fn map_effective_diameter_error(
    err: HelixError,
) -> ApplicationError {

    match err {

        HelixError::ToolTooLarge { .. } =>
            single_field_error(
                "toolDiameter",
                "invalid_combination",
                err.to_string(),
            ),

        HelixError::EffectiveDiameterNotPositive { .. } =>
            single_field_error(
                "diameter",
                "invalid_combination",
                err.to_string(),
            ),

        other =>
            single_field_error(
                "helix",
                "invalid_geometry",
                other.to_string(),
            ),
    }
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