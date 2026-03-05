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
                diameter,
                tool_diameter,
                pitch,
            } => SolveHelixInput::Pitch {
                mode: mode.into(),
                diameter,
                tool_diameter,
                pitch,
            },

            SolveHelixRequest::Angle {
                mode,
                diameter,
                tool_diameter,
                angle,
            } => SolveHelixInput::Angle {
                mode: mode.into(),
                diameter,
                tool_diameter,
                angle,
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
            pitch: out.pitch,
            angle: out.angle,
        }
    }
}
