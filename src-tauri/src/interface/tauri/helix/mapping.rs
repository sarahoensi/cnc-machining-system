//! Mapping between helix Tauri DTOs and application DTOs.
//!
//! This module translates frontend representation types into application input
//! and converts application output into frontend response DTOs.

// interface/tauri/helix/mapping.rs

use crate::application::{
    SolveHelixInput,
    SolveHelixOutput,
    HelixMode as AppHelixMode,
};

use super::{
    SolveHelixRequest,
    SolveHelixResponse,
    HelixMode as UiHelixMode,
};

// ---------------------------------------------------------
// Request → Application Input
// ---------------------------------------------------------

impl From<UiHelixMode> for AppHelixMode {
    fn from(mode: UiHelixMode) -> Self {
        match mode {
            UiHelixMode::Inner => AppHelixMode::Inner,
            UiHelixMode::Outer => AppHelixMode::Outer,
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
            effective_diameter_mm: out.effective_diameter_mm,
            pitch_mm_per_rev: out.pitch_mm_per_rev,
            angle_deg: out.angle_deg,
            circumference_mm: out.circumference_mm,
        }
    }
}
