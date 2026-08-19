use crate::application::{
    SolveThreadInput, SolveThreadOutput, ThreadOptionsOutput, ThreadPitchOptionOutput,
    ThreadSizeOptionOutput, ThreadTypeOptionOutput,
};

use super::{
    SolveThreadRequest, SolveThreadResponse, ThreadOptionsResponse, ThreadPitchOptionResponse,
    ThreadSizeOptionResponse, ThreadTypeOptionResponse,
};

impl From<SolveThreadRequest> for SolveThreadInput {
    fn from(request: SolveThreadRequest) -> Self {
        Self {
            thread_type: request.thread_type,
            size: request.size,
            pitch: request.pitch,
        }
    }
}

impl From<SolveThreadOutput> for SolveThreadResponse {
    fn from(output: SolveThreadOutput) -> Self {
        Self {
            drill_diameter_mm: output.drill_diameter_mm,
            thread_depth_mm: output.thread_depth_mm,
        }
    }
}

impl From<ThreadOptionsOutput> for ThreadOptionsResponse {
    fn from(options: ThreadOptionsOutput) -> Self {
        Self {
            types: options.types.into_iter().map(Into::into).collect(),
            metric: options.metric.into_iter().map(Into::into).collect(),
            unc: options.unc.into_iter().map(Into::into).collect(),
            unf: options.unf.into_iter().map(Into::into).collect(),
            bsp: options.bsp.into_iter().map(Into::into).collect(),
            npt: options.npt.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ThreadTypeOptionOutput> for ThreadTypeOptionResponse {
    fn from(option: ThreadTypeOptionOutput) -> Self {
        Self {
            value: option.value,
            label: option.label,
        }
    }
}

impl From<ThreadSizeOptionOutput> for ThreadSizeOptionResponse {
    fn from(option: ThreadSizeOptionOutput) -> Self {
        Self {
            value: option.value,
            label: option.label,
            major_diameter_mm: option.major_diameter_mm,
            pitches: option.pitches.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ThreadPitchOptionOutput> for ThreadPitchOptionResponse {
    fn from(option: ThreadPitchOptionOutput) -> Self {
        Self {
            value: option.value,
            label: option.label,
            pitch_mm: option.pitch_mm,
            series: option.series,
            is_default_pitch: option.is_default_pitch,
            tap_drill_basis: option.tap_drill_basis,
        }
    }
}
