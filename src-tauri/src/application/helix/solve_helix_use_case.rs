//! Use case for helix parameter orchestration in machining workflows.

use std::f64::consts::PI;

use crate::application::shared::AppResult;
use crate::application::{ApplicationError, ValidationErrors};

use crate::application::helix::dto::{SolveHelixInput, SolveHelixOutput};

use crate::domain::{
    units::{Angle, Diameter, Pitch},
    EffectiveDiameter, Helix, HelixAngle,
};

pub struct SolveHelixUseCase;

impl SolveHelixUseCase {
    pub fn execute(&self, input: SolveHelixInput) -> AppResult<SolveHelixOutput> {
        let helix = self.solve_helix(input)?;
        Ok(helix.into())
    }

    fn solve_helix(&self, input: SolveHelixInput) -> AppResult<Helix> {
        match input {
            SolveHelixInput::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => {
                // 1) valider alle inputfelt (samle)
                let (nominal, tool, pitch) = parse_pitch_inputs(
                    diameter_mm,
                    tool_diameter_mm,
                    pitch_mm_per_rev,
                )?;

                // 2) domain (fail-fast)
                let effective = EffectiveDiameter::new(mode.into(), nominal, tool)?.diameter();
                Ok(Helix::new(effective, pitch))
            }

            SolveHelixInput::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => {
                // 1) valider alle inputfelt (samle)
                let (nominal, tool, angle) =
                    parse_angle_inputs(diameter_mm, tool_diameter_mm, angle_deg)?;

                // 2) domain (fail-fast)
                let effective = EffectiveDiameter::new(mode.into(), nominal, tool)?.diameter();

                let helix_angle = HelixAngle::new(angle)?;

                // pitch = tan(angle) * circumference
                let circumference = PI * effective.mm_value();
                let pitch_value = helix_angle.radians_value().tan() * circumference;

                let pitch = Pitch::mm_per_rev(pitch_value)?; // UnitError -> ApplicationError::Unit via From

                Ok(Helix::new(effective, pitch))
            }
        }
    }
}

// ---------------------------------------------------------
// Parse helpers (samler ValidationErrors)
// ---------------------------------------------------------

fn parse_pitch_inputs(
    diameter_mm: f64,
    tool_diameter_mm: f64,
    pitch_mm_per_rev: f64,
) -> Result<(Diameter, Diameter, Pitch), ApplicationError> {
    let mut errors = ValidationErrors::new();

    let nominal = match Diameter::mm(diameter_mm) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push("diameter", "invalid", e.to_string());
            None
        }
    };

    let tool = match Diameter::mm(tool_diameter_mm) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push("toolDiameter", "invalid", e.to_string());
            None
        }
    };

    let pitch = match Pitch::mm_per_rev(pitch_mm_per_rev) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push("pitch", "invalid", e.to_string());
            None
        }
    };

    if !errors.is_empty() {
        return Err(ApplicationError::Validation(errors));
    }

    Ok((nominal.unwrap(), tool.unwrap(), pitch.unwrap()))
}

fn parse_angle_inputs(
    diameter_mm: f64,
    tool_diameter_mm: f64,
    angle_deg: f64,
) -> Result<(Diameter, Diameter, Angle), ApplicationError> {
    let mut errors = ValidationErrors::new();

    let nominal = match Diameter::mm(diameter_mm) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push("diameter", "invalid", e.to_string());
            None
        }
    };

    let tool = match Diameter::mm(tool_diameter_mm) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push("toolDiameter", "invalid", e.to_string());
            None
        }
    };

    let angle = match Angle::degrees(angle_deg) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push("angle", "invalid", e.to_string());
            None
        }
    };

    if !errors.is_empty() {
        return Err(ApplicationError::Validation(errors));
    }

    Ok((nominal.unwrap(), tool.unwrap(), angle.unwrap()))
}