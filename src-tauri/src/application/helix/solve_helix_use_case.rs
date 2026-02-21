//! Use case for helix parameter orchestration in machining workflows.

use crate::application::helix::dto::{
    SolveHelixInput,
    SolveHelixOutput,
};

use crate::application::shared::AppResult;
use crate::application::{ApplicationError, ValidationErrors};


use crate::domain::{
    units::{Angle, Diameter, Pitch},
    Helix,
    HelixAngle,
    EffectiveDiameter,
};

use std::f64::consts::PI;

pub struct SolveHelixUseCase;

impl SolveHelixUseCase {

    pub fn execute(
        &self,
        input: SolveHelixInput,
    ) -> AppResult<SolveHelixOutput> {

        let helix = self.parse_and_solve(input)?;

        Ok(helix.into())
    }

    // ---------------------------------------------------------
    // Step 1: Parse + validate
    // ---------------------------------------------------------

    fn parse_and_solve(
        &self,
        input: SolveHelixInput,
    ) -> Result<Helix, ApplicationError> {

        let mut errors = ValidationErrors::new();

        match input {

            // ---------------------------------------------
            // Pitch path
            // ---------------------------------------------
            SolveHelixInput::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => {

                let nominal = match Diameter::mm(diameter_mm) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push("diameter_mm", "invalid", e.to_string());
                        return Err(ApplicationError::Validation(errors));
                    }
                };

                let tool = match Diameter::mm(tool_diameter_mm) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push("tool_diameter_mm", "invalid", e.to_string());
                        return Err(ApplicationError::Validation(errors));
                    }
                };

                let pitch = match Pitch::mm_per_rev(pitch_mm_per_rev) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push("pitch_mm_per_rev", "invalid", e.to_string());
                        return Err(ApplicationError::Validation(errors));
                    }
                };

                let effective = EffectiveDiameter::new(
                    mode.into(),
                    nominal,
                    tool,
                )?;

                Ok(Helix::new(effective.diameter(), pitch))
            }

            // ---------------------------------------------
            // Angle path
            // ---------------------------------------------
            SolveHelixInput::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => {

                let nominal = match Diameter::mm(diameter_mm) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push("diameter_mm", "invalid", e.to_string());
                        return Err(ApplicationError::Validation(errors));
                    }
                };

                let tool = match Diameter::mm(tool_diameter_mm) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push("tool_diameter_mm", "invalid", e.to_string());
                        return Err(ApplicationError::Validation(errors));
                    }
                };

                let angle = match Angle::degrees(angle_deg) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push("angle_deg", "invalid", e.to_string());
                        return Err(ApplicationError::Validation(errors));
                    }
                };

                let helix_angle = HelixAngle::new(angle)?;

                let effective = EffectiveDiameter::new(
                    mode.into(),
                    nominal,
                    tool,
                )?;

                let circumference = PI * effective.diameter().mm_value();
                let pitch_value = helix_angle.radians_value().tan() * circumference;

                let pitch = Pitch::mm_per_rev(pitch_value)?;

                Ok(Helix::new(effective.diameter(), pitch))
            }
        }
    }
}