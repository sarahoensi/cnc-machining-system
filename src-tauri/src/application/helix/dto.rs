//! DTOs for helix-solving workflows.
//!
//! These types define application-facing input modes and normalized output for
//! helix geometry orchestration used by machining interfaces.

// application/helix/dto.rs

use crate::domain::Helix;

#[derive(Debug, Copy, Clone)]
/// Indicates how tool diameter offsets are applied to nominal diameter.
///
/// This mode determines whether the effective helix diameter is computed for
/// an inner or outer machining path.
pub enum HelixMode {
    /// Inner path: effective diameter is reduced by tool radius offset.
    Inner,
    /// Outer path: effective diameter is increased by tool radius offset.
    Outer,
}

/// Input DTO for solving helix parameters.
///
/// This is an application input contract. The caller provides either pitch or
/// helix angle together with machining context values in millimeters.
///
/// Validation expectations:
/// - Diameter and tool diameter must satisfy domain diameter constraints.
/// - Pitch or angle must satisfy domain constraints for helix construction.
///
/// Unit expectations:
/// - Diameters in millimeters (`mm`).
/// - Pitch in millimeters per revolution (`mm/rev`).
/// - Angle in degrees (`deg`).
pub enum SolveHelixInput {

    /// Solve helix state from known pitch.
    Pitch {
        /// Effective-diameter calculation mode.
        mode: HelixMode,
        /// Nominal path diameter (`mm`).
        diameter_mm: f64,
        /// Tool diameter used to offset the nominal path (`mm`).
        tool_diameter_mm: f64,
        /// Helix pitch (`mm/rev`).
        pitch_mm_per_rev: f64,
    },

    /// Solve helix state from known helix angle.
    Angle {
        /// Effective-diameter calculation mode.
        mode: HelixMode,
        /// Nominal path diameter (`mm`).
        diameter_mm: f64,
        /// Tool diameter used to offset the nominal path (`mm`).
        tool_diameter_mm: f64,
        /// Helix angle (`deg`).
        angle_deg: f64,
    },
}

/// Output DTO for a solved helix.
///
/// This is an application output contract returned to external layers after
/// domain helix construction and normalization.
pub struct SolveHelixOutput {
    /// Effective helix diameter used by the solved model (`mm`).
    pub effective_diameter_mm: f64,
    /// Solved helix pitch (`mm/rev`).
    pub pitch_mm_per_rev: f64,
    /// Solved helix angle (`deg`).
    pub angle_deg: f64,
    /// Circumference at effective diameter (`mm`).
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
