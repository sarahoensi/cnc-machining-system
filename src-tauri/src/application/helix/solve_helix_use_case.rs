//! Use case for helix parameter orchestration in machining workflows.
//!
//! This module coordinates application inputs with domain helix construction,
//! supporting pitch-driven and angle-driven solution paths.

// application/helix/solve_helix_use_case.rs
use crate::application::helix::dto::{
    SolveHelixInput,
    SolveHelixOutput,
    HelixMode,
};

use crate::application::shared::AppResult;

use crate::domain::{
    units::{Angle, Diameter,  Pitch},
    Helix, HelixAngle,
};


use std::f64::consts::PI;

pub struct SolveHelixUseCase;

impl SolveHelixUseCase {

    /// Solves helix parameters for an inner or outer machining path.
    ///
    /// Purpose:
    /// - Orchestrates effective-diameter setup and helix construction from the
    ///   selected input mode.
    ///
    /// Required inputs:
    /// - A valid [`SolveHelixInput`] variant with diameters in millimeters and
    ///   either pitch (`mm/rev`) or angle (`deg`).
    ///
    /// Output meaning:
    /// - Returns normalized helix values in [`SolveHelixOutput`] for use by UI
    ///   or API layers.
    ///
    /// Domain invariants enforced:
    /// - Diameter, angle, and pitch constraints are validated by domain value
    ///   objects and helix model construction.
    ///
    /// Side effects:
    /// - None. This use case does not perform persistence.
    ///
    /// Error scenarios:
    /// - Invalid numeric/unit inputs rejected by domain constructors.
    /// - Domain helix construction failures for unsupported value combinations.
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
