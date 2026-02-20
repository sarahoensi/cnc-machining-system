//! Mapping between helix Tauri DTOs and application DTOs.
//!
//! This module translates frontend representation types into application input
//! and converts application output into frontend response DTOs.

// interface/tauri/helix/mapping.rs

use crate::application::{
    SolveHelixInput,
    SolveHelixOutput,
};

use super::{
    SolveHelixRequest,
    SolveHelixResponse,
    HelixMode as UiHelixMode,
};

use crate::domain::HelixMode;

// ---------------------------------------------------------
// Request → Application Input
// ---------------------------------------------------------

impl From<UiHelixMode> for HelixMode {
    fn from(mode: UiHelixMode) -> Self {
        match mode {
            UiHelixMode::Inner => HelixMode::Inner,
            UiHelixMode::Outer => HelixMode::Outer,
        }
    }
}

impl From<SolveHelixRequest> for SolveHelixInput {
    fn from(req: SolveHelixRequest) -> Self {
        match req {

            SolveHelixRequest::Pitch {
                mode,
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            } => SolveHelixInput::Pitch {
                mode: mode.into(),
                diameter_mm,
                tool_diameter_mm,
                pitch_mm_per_rev,
            },

            SolveHelixRequest::Angle {
                mode,
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            } => SolveHelixInput::Angle {
                mode: mode.into(),
                diameter_mm,
                tool_diameter_mm,
                angle_deg,
            },
        }
    }
}

// ---------------------------------------------------------
// Application Output → Response
// ---------------------------------------------------------

impl From<SolveHelixOutput> for SolveHelixResponse {
    fn from(out: SolveHelixOutput) -> Self {
        Self {
            pitch_mm_per_rev: out.pitch_mm_per_rev,
            angle_deg: out.angle_deg,
        }
    }
}
