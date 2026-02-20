//! Use case for helix parameter orchestration in machining workflows.

use crate::application::helix::dto::{
    SolveHelixInput,
    SolveHelixOutput,
};

use crate::application::shared::AppResult;

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

        let helix = self.solve_helix(input)?;

        Ok(helix.into())
    }

    // ---------------------------------------------------------
    // Internal workflow
    // ---------------------------------------------------------

    fn solve_helix(
        &self,
        input: SolveHelixInput,
    ) -> AppResult<Helix> {

        match input {

            // ---------------------------------------------
            // Solve from pitch
            // ---------------------------------------------
            SolveHelixInput::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => {

                let nominal = Diameter::mm(diameter_mm)?;
                let tool = Diameter::mm(tool_diameter_mm)?;

                let effective = EffectiveDiameter::new(
                    mode.into(),
                    nominal,
                    tool,
                )?.diameter();

                let pitch = Pitch::mm_per_rev(pitch_mm_per_rev)?;

                Ok(Helix::new(effective, pitch))
            }

            // ---------------------------------------------
            // Solve from angle
            // ---------------------------------------------
            SolveHelixInput::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => {

                let nominal = Diameter::mm(diameter_mm)?;
                let tool = Diameter::mm(tool_diameter_mm)?;

                let effective = EffectiveDiameter::new(
                    mode.into(),
                    nominal,
                    tool,
                )?.diameter();

                let angle = Angle::degrees(angle_deg)?;
                let helix_angle = HelixAngle::new(angle)?;

                // pitch = tan(angle) * circumference
                let circumference = PI * effective.mm_value();
                let pitch_value = helix_angle.radians_value().tan() * circumference;

                let pitch = Pitch::mm_per_rev(pitch_value)?;

                Ok(Helix::new(effective, pitch))
            }
        }
    }
}