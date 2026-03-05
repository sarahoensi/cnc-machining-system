// application/helix/solve_helix_use_case.rs

use crate::application::{
    helix::dto::{SolveHelixInput, SolveHelixOutput},
    shared::{AppResult, InputParser},
};

use crate::domain::{
    units::{AcuteAngle, Diameter, Pitch},
    Helix,
};

pub struct SolveHelixUseCase;

impl SolveHelixUseCase {
    pub fn execute(&self, input: SolveHelixInput) -> AppResult<SolveHelixOutput> {
        let mut p = InputParser::new();

        let helix = match input {
            SolveHelixInput::Pitch {
                mode,
                diameter,
                tool_diameter,
                pitch,
            } => {
                let diameter = p.value("diameter", Diameter::mm(diameter));

                let tool = p.value("tool_diameter", Diameter::mm(tool_diameter));

                let pitch = p.value("pitch", Pitch::mm_per_rev(pitch));

                match (diameter, tool, pitch) {
                    (Some(d), Some(t), Some(pitch)) => {
                        p.domain("toolDiameter", Helix::from_pitch(mode.into(), d, t, pitch))
                    }
                    _ => None,
                }
            }

            SolveHelixInput::Angle {
                mode,
                diameter,
                tool_diameter,
                angle,
            } => {
                let diameter = p.value("diameter", Diameter::mm(diameter));

                let tool = p.value("tool_diameter", Diameter::mm(tool_diameter));

                let angle = p.value("angle", AcuteAngle::degrees(angle));

                match (diameter, tool, angle) {
                    (Some(d), Some(t), Some(a)) => {
                        p.domain("toolDiameter", Helix::from_angle(mode.into(), d, t, a))
                    }
                    _ => None,
                }
            }
        };

        let helix = p.finish_with(helix)?;

        Ok(helix.into())
    }
}
