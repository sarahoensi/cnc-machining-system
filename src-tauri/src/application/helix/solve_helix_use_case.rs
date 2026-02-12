// application/helix/solve_helix_use_case.rs
use crate::application::helix::dto::{
    SolveHelixInput,
    SolveHelixOutput,
    HelixMode,
};

use crate::application::shared::AppResult;

use crate::domain::{
    Angle,
    Diameter,
    Helix,
    HelixAngle,
};

use crate::domain::Pitch;

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

            // -------------------------------------------------
            // Solve from pitch
            // -------------------------------------------------
            SolveHelixInput::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => {

                let effective = self.effective_diameter(
                    mode,
                    diameter_mm,
                    tool_diameter_mm,
                )?;

                let pitch = Pitch::mm_per_rev(pitch_mm_per_rev)?;

                Ok(Helix::new(effective, pitch))
            }

            // -------------------------------------------------
            // Solve from angle
            // -------------------------------------------------
            SolveHelixInput::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => {

                let effective = self.effective_diameter(
                    mode,
                    diameter_mm,
                    tool_diameter_mm,
                )?;

                let angle = Angle::degrees(angle_deg)?;
                let helix_angle = HelixAngle::new(angle)?;

                let pitch = self.pitch_from_angle(effective, helix_angle)?;

                Ok(Helix::new(effective, pitch))
            }
        }
    }

    // ---------------------------------------------------------
    // Application helpers
    // ---------------------------------------------------------

    fn effective_diameter(
        &self,
        mode: HelixMode,
        nominal_mm: f64,
        tool_mm: f64,
    ) -> AppResult<Diameter> {

        let nominal = Diameter::mm(nominal_mm)?;
        let tool = Diameter::mm(tool_mm)?;

        let offset = tool.mm_value() / 2.0;

        let value = match mode {
            HelixMode::Outer => nominal.mm_value() + offset,
            HelixMode::Inner => nominal.mm_value() - offset,
        };

        Ok(Diameter::mm(value)?)
    }

    fn pitch_from_angle(
        &self,
        diameter: Diameter,
        angle: HelixAngle,
    ) -> AppResult<Pitch> {

        let circumference = PI * diameter.mm_value();

        let pitch = angle.radians_value().tan() * circumference;

        Ok(Pitch::mm_per_rev(pitch)?)
    }
}

// --------- TESTS ----
#[cfg(test)]
mod tests {

    use super::*;
    use crate::application::helix::dto::{HelixMode, SolveHelixInput};

    #[test]
    fn solves_from_pitch_outer() {

        let use_case = SolveHelixUseCase;

        let input = SolveHelixInput::Pitch {
            mode: HelixMode::Outer,
            diameter_mm: 10.0,
            tool_diameter_mm: 2.0,
            pitch_mm_per_rev: 4.0,
        };

        let result = use_case.execute(input).unwrap();

        assert!(result.pitch_mm_per_rev > 0.0);
    }

    #[test]
    fn solves_from_angle_inner() {

        let use_case = SolveHelixUseCase;

        let input = SolveHelixInput::Angle {
            mode: HelixMode::Inner,
            diameter_mm: 10.0,
            tool_diameter_mm: 2.0,
            angle_deg: 20.0,
        };

        let result = use_case.execute(input).unwrap();

        assert!(result.pitch_mm_per_rev > 0.0);
    }
}
