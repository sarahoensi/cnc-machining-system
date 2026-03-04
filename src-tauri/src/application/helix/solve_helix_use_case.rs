// application/helix/solve_helix_use_case.rs

// application/helix/solve_helix_use_case.rs

use crate::application::shared::AppResult;

use crate::application::helix::dto::{SolveHelixInput, SolveHelixOutput};

use crate::domain::{
    units::{AcuteAngle, Diameter, Pitch},
    DomainError, Helix,
};

pub struct SolveHelixUseCase;

impl SolveHelixUseCase {
    // ---------------------------------------------------------
    // Public entrypoint (Application boundary)
    // ---------------------------------------------------------

    pub fn execute(&self, input: SolveHelixInput) -> AppResult<SolveHelixOutput> {
        let helix = self.solve_helix(input)?;
        Ok(helix.into())
    }

    // ---------------------------------------------------------
    // Internal orchestration (Domain boundary)
    // ---------------------------------------------------------

    fn solve_helix(&self, input: SolveHelixInput) -> Result<Helix, DomainError> {
        match input {
            SolveHelixInput::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => Ok(Helix::from_pitch(
                mode.into(),
                Diameter::mm(diameter_mm)?,
                Diameter::mm(tool_diameter_mm)?,
                Pitch::mm_per_rev(pitch_mm_per_rev)?,
            )?),

            SolveHelixInput::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => Ok(Helix::from_angle(
                mode.into(),
                Diameter::mm(diameter_mm)?,
                Diameter::mm(tool_diameter_mm)?,
                AcuteAngle::degrees(angle_deg)?,
            )?),
        }
    }
}
