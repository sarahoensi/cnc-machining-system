//! Use case for solving and completing cutting data.
//!
//! - Parses raw operator input into validated domain value objects.
//! - Delegates mathematical consistency to `MachiningSolver`.
//! - Maps domain errors back into field-level `ValidationErrors`.
//!
//! This use case is pure orchestration (no persistence, no side effects).

use crate::application::shared::AppResult;
use crate::application::{ApplicationError, ValidationErrors};

use crate::application::cutting_data::dto::{SolveCuttingDataInput, SolveCuttingDataOutput};

use crate::domain::{
    units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm},
    MachiningPhysicsError, MachiningSolver, Tool, ToothCount,
};

pub struct SolveCuttingDataUseCase;

impl SolveCuttingDataUseCase {
    /// Entry point for solving cutting data.
    pub fn execute(input: SolveCuttingDataInput) -> AppResult<SolveCuttingDataOutput> {
        let mut v = ValidationErrors::new();

        // ---------------------------------------------------------
        // Parse raw inputs into domain value objects
        // ---------------------------------------------------------

        let cutting_speed = parse_opt_f64(
            "cutting_speed_m_per_min",
            input.cutting_speed_m_per_min,
            CuttingSpeed::meters_per_min,
            &mut v,
        );

        let rpm = parse_opt_f64("rpm", input.rpm, Rpm::new, &mut v);

        let diameter = parse_opt_f64("diameter_mm", input.diameter_mm, Diameter::mm, &mut v);

        let chip = parse_opt_f64(
            "chip_load_mm_per_tooth",
            input.chip_load_mm_per_tooth,
            ChipLoad::mm_per_tooth,
            &mut v,
        );

        let feed = parse_opt_f64(
            "feed_rate_mm_per_min",
            input.feed_rate_mm_per_min,
            FeedRate::mm_per_min,
            &mut v,
        );

        let teeth = parse_opt_u32("teeth", input.teeth, ToothCount::new, &mut v);

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        // ---------------------------------------------------------
        // Tool is required for any cross-parameter calculation
        // ---------------------------------------------------------

        let tool = match (diameter, teeth) {
            (Some(d), Some(z)) => Tool::new(d, z),
            _ => {
                return Ok(to_output_partial(cutting_speed, rpm, chip, feed));
            }
        };

        // ---------------------------------------------------------
        // Delegate to domain solver
        // ---------------------------------------------------------

        let params = match (cutting_speed, chip, rpm, feed) {
            // Cutting speed + chip load
            (Some(vc), Some(fz), _, _) => MachiningSolver::from_speed_and_chip_load(vc, fz, tool),

            // RPM + feed rate
            (_, _, Some(n), Some(f)) => MachiningSolver::from_rpm_and_feed(n, f, tool),

            // Insufficient combination
            _ => {
                return Ok(to_output_partial(cutting_speed, rpm, chip, feed));
            }
        }
        .map_err(map_machining_error)?;

        // ---------------------------------------------------------
        // Convert domain result to DTO
        // ---------------------------------------------------------

        Ok(SolveCuttingDataOutput {
            cutting_speed_m_per_min: Some(params.cutting_speed().meters_per_min_value()),
            rpm: Some(params.rpm().value()),
            chip_load_mm_per_tooth: Some(params.chip_load().mm_per_tooth_value()),
            feed_rate_mm_per_min: Some(params.feed_rate().mm_per_min_value()),
        })
    }
}

// ---------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------

fn parse_opt_f64<T, E, F>(
    field: &'static str,
    raw: Option<f64>,
    ctor: F,
    v: &mut ValidationErrors,
) -> Option<T>
where
    F: FnOnce(f64) -> Result<T, E>,
    E: std::error::Error,
{
    match raw {
        Some(value) => match ctor(value) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(field, "invalid", e.to_string());
                None
            }
        },
        None => None,
    }
}

fn parse_opt_u32<T, E, F>(
    field: &'static str,
    raw: Option<u32>,
    ctor: F,
    v: &mut ValidationErrors,
) -> Option<T>
where
    F: FnOnce(u32) -> Result<T, E>,
    E: std::error::Error,
{
    match raw {
        Some(value) => match ctor(value) {
            Ok(val) => Some(val),
            Err(e) => {
                v.push(field, "invalid", e.to_string());
                None
            }
        },
        None => None,
    }
}

// ---------------------------------------------------------
// Domain error → ValidationErrors mapping
// ---------------------------------------------------------

fn map_machining_error(err: MachiningPhysicsError) -> ApplicationError {
    let mut v = ValidationErrors::new();

    match err {
        MachiningPhysicsError::InvalidDiameter { .. } => {
            v.push("diameter_mm", "invalid", err.to_string());
        }

        MachiningPhysicsError::InvalidToothCount { .. } => {
            v.push("teeth", "invalid", err.to_string());
        }

        MachiningPhysicsError::InvalidRpm { .. } => {
            v.push("rpm", "invalid_combination", err.to_string());
        }

        MachiningPhysicsError::InvalidFeedRate { .. } => {
            v.push(
                "feed_rate_mm_per_min",
                "invalid_combination",
                err.to_string(),
            );
        }

        MachiningPhysicsError::InvalidChipLoad { .. } => {
            v.push(
                "chip_load_mm_per_tooth",
                "invalid_combination",
                err.to_string(),
            );
        }

        MachiningPhysicsError::DivisionByZero | MachiningPhysicsError::NumericalInstability => {
            v.push("cutting_data", "invalid_combination", err.to_string());
        }
    }

    ApplicationError::Validation(v)
}

// ---------------------------------------------------------
// Partial output helper
// ---------------------------------------------------------

fn to_output_partial(
    cutting_speed: Option<CuttingSpeed>,
    rpm: Option<Rpm>,
    chip: Option<ChipLoad>,
    feed: Option<FeedRate>,
) -> SolveCuttingDataOutput {
    SolveCuttingDataOutput {
        cutting_speed_m_per_min: cutting_speed.map(|v| v.meters_per_min_value()),

        rpm: rpm.map(|r| r.value()),

        chip_load_mm_per_tooth: chip.map(|c| c.mm_per_tooth_value()),

        feed_rate_mm_per_min: feed.map(|f| f.mm_per_min_value()),
    }
}
