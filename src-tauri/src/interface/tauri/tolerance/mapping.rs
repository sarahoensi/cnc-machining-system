use crate::application::{
    FitResult, FitSummary, ToleranceOption, ToleranceOptions, ToleranceResult,
};

use super::{
    FitResponse, FitSummaryResponse, ToleranceOptionResponse, ToleranceOptionsResponse,
    ToleranceResponse,
};

impl From<ToleranceResult> for ToleranceResponse {
    fn from(result: ToleranceResult) -> Self {
        Self {
            code: result.code,
            zone: result.zone,
            grade: result.grade,
            upper_um: result.upper_um,
            lower_um: result.lower_um,
            min_mm: result.min_mm,
            max_mm: result.max_mm,
            source_table: result.source_table,
            source_file: result.source_file,
        }
    }
}

impl From<ToleranceOption> for ToleranceOptionResponse {
    fn from(option: ToleranceOption) -> Self {
        Self {
            feature: option.feature,
            zone: option.zone,
            grades: option.grades,
        }
    }
}

impl From<ToleranceOptions> for ToleranceOptionsResponse {
    fn from(options: ToleranceOptions) -> Self {
        Self {
            holes: options.holes.into_iter().map(Into::into).collect(),
            shafts: options.shafts.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<FitSummary> for FitSummaryResponse {
    fn from(summary: FitSummary) -> Self {
        Self {
            min_clearance_mm: summary.min_clearance_mm,
            max_clearance_mm: summary.max_clearance_mm,
            fit_type: summary.fit_type,
        }
    }
}

impl From<FitResult> for FitResponse {
    fn from(result: FitResult) -> Self {
        Self {
            nominal_mm: result.nominal_mm,
            hole: result.hole.into(),
            shaft: result.shaft.into(),
            fit: result.fit.into(),
        }
    }
}
