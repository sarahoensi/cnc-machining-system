use std::f64::consts::PI;

use crate::application::shared::{AppResult};


use crate::application::helix::dto::{
    SolveHelixInput,
    SolveHelixOutput,
};

use crate::domain::{
    DomainError,
    units::{AcuteAngle, Diameter, Pitch},
    EffectiveDiameter,
    Helix,

};

pub struct SolveHelixUseCase;

impl SolveHelixUseCase {

    // ---------------------------------------------------------
    // Public entrypoint (Application boundary)
    // ---------------------------------------------------------

    pub fn execute(
        &self,
        input: SolveHelixInput,
    ) -> AppResult<SolveHelixOutput> {

        let helix = self.solve_helix(input)?;
        Ok(helix.into())
    }

    // ---------------------------------------------------------
    // Internal orchestration (Domain boundary)
    // ---------------------------------------------------------

    fn solve_helix(
        &self,
        input: SolveHelixInput,
    ) -> Result<Helix, DomainError> {

        match input {

            SolveHelixInput::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => {

                let diameter = Diameter::mm(diameter_mm)?;
                let tool = Diameter::mm(tool_diameter_mm)?;
                let pitch = Pitch::mm_per_rev(pitch_mm_per_rev)?;

                let effective =
                    EffectiveDiameter::new(mode.into(), diameter, tool)?;

                Ok(Helix::new(
                    effective.diameter(),
                    pitch,
                ))
            }

            SolveHelixInput::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => {

                let diameter = Diameter::mm(diameter_mm)?;
                let tool = Diameter::mm(tool_diameter_mm)?;
                let angle = AcuteAngle::degrees(angle_deg)?;

                let effective =
                    EffectiveDiameter::new(mode.into(), diameter, tool)?;

                // pitch = tan(angle) * circumference
                let circumference =
                    PI * effective.diameter().mm_value();

                let pitch_value =
                    angle.radians_value().tan() * circumference;

                let pitch =
                    Pitch::mm_per_rev(pitch_value)?;

                Ok(Helix::new(
                    effective.diameter(),
                    pitch,
                ))
            }
        }
    }
}