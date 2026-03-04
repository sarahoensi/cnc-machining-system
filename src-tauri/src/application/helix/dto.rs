// application/helix/dto.rs
use crate::domain::{Helix, HelixMode};

pub enum SolveHelixInput {
    Pitch {
        mode: HelixMode,
        diameter_mm: f64,
        tool_diameter_mm: f64,
        pitch_mm_per_rev: f64,
    },

    Angle {
        mode: HelixMode,
        diameter_mm: f64,
        tool_diameter_mm: f64,
        angle_deg: f64,
    },
}

pub struct SolveHelixOutput {
    pub effective_diameter_mm: f64,
    pub pitch_mm_per_rev: f64,
    pub angle_deg: f64,
    pub circumference_mm: f64,
}

// ---------------------------------------------------------
// Domain → Application DTO mapping
// ---------------------------------------------------------

impl From<Helix> for SolveHelixOutput {
    fn from(helix: Helix) -> Self {
        let pitch = helix.pitch();
        let angle = helix.helix_angle();

        Self {
            effective_diameter_mm: helix.diameter().mm_value(),
            pitch_mm_per_rev: pitch.mm_per_rev_value(),
            angle_deg: angle.degrees_value(),
            circumference_mm: helix.circumference().mm_value(),
        }
    }
}