// application/helix/dto.rs
use crate::domain::{Helix, HelixMode};

pub enum SolveHelixInput {
    Pitch {
        mode: HelixMode,
        diameter: f64,
        tool_diameter: f64,
        pitch: f64,
    },

    Angle {
        mode: HelixMode,
        diameter: f64,
        tool_diameter: f64,
        angle: f64,
    },
}

pub struct SolveHelixOutput {
    //pub effective_diameter: f64,
    pub pitch: f64,
    pub angle: f64,
    //pub circumference: f64,
}

// ---------------------------------------------------------
// Domain → Application DTO mapping
// ---------------------------------------------------------

impl From<Helix> for SolveHelixOutput {
    fn from(helix: Helix) -> Self {
        let pitch = helix.pitch();
        let angle = helix.helix_angle();

        Self {
            //effective_diameter: helix.diameter().mm_value(),
            pitch: pitch.mm_per_rev_value(),
            angle: angle.degrees_value(),
            //circumference: helix.circumference().mm_value(),
        }
    }
}