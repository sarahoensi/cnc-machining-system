//! Use case for solving and completing cutting data.
//!
//! The workflow coordinates domain calculators to infer missing machining
//! parameters from a partial input set while preserving domain validation.

// application/cutting_data/solve_cutting_data_use_case.rs

use crate::application::{ApplicationError, ValidationErrors};
use crate::application::shared::AppResult;

use crate::application::cutting_data::dto::{SolveCuttingDataInput, SolveCuttingDataOutput};

use crate::domain::{
    units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm},
    ChipLoadCalculator, FeedRateCalculator, SpindleSpeedCalculator, ToothCount,
};

pub struct SolveCuttingDataUseCase;

impl SolveCuttingDataUseCase {
    /// Completes a cutting-data set from the provided partial input.
    ///
    /// Purpose:
    /// - Orchestrates spindle speed, cutting speed, feed rate, and chip-load
    ///   relationships used in milling setup workflows.
    ///
    /// Required inputs:
    /// - A valid subset of fields in [`SolveCuttingDataInput`].
    /// - `diameter_mm` is needed for speed conversions.
    /// - `teeth` is needed for feed/chip-load conversions.
    ///
    /// Output guarantees:
    /// - Returns validated and derived values in [`SolveCuttingDataOutput`].
    /// - Leaves unresolved fields as `None` when data is insufficient.
    ///
    /// Domain invariants enforced:
    /// - Unit/domain value objects validate positivity and allowed ranges.
    /// - Formula consistency is delegated to domain calculator services.
    ///
    /// Side effects:
    /// - None. This use case is pure orchestration with no persistence.
    ///
    /// Error scenarios:
    /// - Returns an application error when any provided value is invalid.
    /// - Returns an application error when a requested domain calculation fails.

    /// Entry point for solving cutting data.
    pub fn execute(input: SolveCuttingDataInput) -> AppResult<SolveCuttingDataOutput> {
        let mut state = Self::parse_input(input)?;

        Self::infer_missing_values(&mut state)?;

        Ok(Self::to_output(state))
    }

    // ---------------------------------------------------------
    // Step 1: Parse input into validated domain state
    // ---------------------------------------------------------

    fn parse_input(input: SolveCuttingDataInput) -> Result<CuttingState, ApplicationError> {
        let mut errors = ValidationErrors::new();

        let cutting_speed = match input.cutting_speed_m_per_min {
            Some(v) => match CuttingSpeed::meters_per_min(v) {
                Ok(val) => Some(val),
                Err(e) => {
                    errors.push("cutting_speed_m_per_min", "invalid", e.to_string());
                    None
                }
            },
            None => None,
        };

        let rpm = match input.rpm {
            Some(v) => match Rpm::new(v) {
                Ok(val) => Some(val),
                Err(e) => {
                    errors.push("rpm", "invalid", e.to_string());
                    None
                }
            },
            None => None,
        };

        let diameter = match input.diameter_mm {
            Some(v) => match Diameter::mm(v) {
                Ok(val) => Some(val),
                Err(e) => {
                    errors.push("diameter_mm", "invalid", e.to_string());
                    None
                }
            },
            None => None,
        };

        let teeth = match input.teeth {
            Some(v) => match ToothCount::new(v) {
                Ok(val) => Some(val),
                Err(e) => {
                    errors.push("teeth", "invalid", e.to_string());
                    None
                }
            },
            None => None,
        };

        let chip_load = match input.chip_load_mm_per_tooth {
    Some(v) => match ChipLoad::mm_per_tooth(v) {
        Ok(val) => Some(val),
        Err(e) => {
            errors.push("chip_load_mm_per_tooth", "invalid", e.to_string());
            None
        }
    },
    None => None,
};

let feed_rate = match input.feed_rate_mm_per_min {
    Some(v) => match FeedRate::mm_per_min(v) {
        Ok(val) => Some(val),
        Err(e) => {
            errors.push("feed_rate_mm_per_min", "invalid", e.to_string());
            None
        }
    },
    None => None,
};

        if !errors.is_empty() {
            return Err(ApplicationError::Validation(errors));
        }

        Ok(CuttingState {
            cutting_speed,
            rpm,
            chip_load,
            feed_rate,
            diameter,
            teeth,
        })
    }

    // ---------------------------------------------------------
    // Step 2: Forward-chaining inference
    // ---------------------------------------------------------

    fn infer_missing_values(state: &mut CuttingState) -> AppResult<()> {
        let mut changed = true;

        while changed {
            changed = false;

            // -------------------------------------------------
            // Cutting speed ↔ RPM
            // -------------------------------------------------

            if state.rpm.is_none() {
                if let (Some(vc), Some(d)) = (state.cutting_speed, state.diameter) {
                    state.rpm = Some(SpindleSpeedCalculator::rpm_from_cutting_speed(vc, d)?);
                    changed = true;
                }
            }

            if state.cutting_speed.is_none() {
                if let (Some(rpm), Some(d)) = (state.rpm, state.diameter) {
                    state.cutting_speed =
                        Some(SpindleSpeedCalculator::cutting_speed_from_rpm(rpm, d)?);
                    changed = true;
                }
            }

            // -------------------------------------------------
            // Chip load ↔ Feed rate
            // -------------------------------------------------

            if state.feed_rate.is_none() {
                if let (Some(chip), Some(rpm), Some(teeth)) =
                    (state.chip_load, state.rpm, state.teeth)
                {
                    state.feed_rate = Some(FeedRateCalculator::feed_rate_from_chip_load(
                        chip, rpm, teeth,
                    )?);
                    changed = true;
                }
            }

            if state.chip_load.is_none() {
                if let (Some(feed), Some(rpm), Some(teeth)) =
                    (state.feed_rate, state.rpm, state.teeth)
                {
                    state.chip_load = Some(ChipLoadCalculator::chip_load_from_feed_rate(
                        feed, rpm, teeth,
                    )?);
                    changed = true;
                }
            }
        }

        Ok(())
    }

    // ---------------------------------------------------------
    // Step 3: Convert back to DTO
    // ---------------------------------------------------------

    fn to_output(state: CuttingState) -> SolveCuttingDataOutput {
        SolveCuttingDataOutput {
            cutting_speed_m_per_min: state.cutting_speed.map(|v| v.meters_per_min_value()),

            rpm: state.rpm.map(|r| r.value()),

            chip_load_mm_per_tooth: state.chip_load.map(|c| c.mm_per_tooth_value()),

            feed_rate_mm_per_min: state.feed_rate.map(|f| f.mm_per_min_value()),
        }
    }
}

// ---------------------------------------------------------
// Internal state representation
// ---------------------------------------------------------

#[derive(Debug)]
struct CuttingState {
    cutting_speed: Option<CuttingSpeed>,
    rpm: Option<Rpm>,
    chip_load: Option<ChipLoad>,
    feed_rate: Option<FeedRate>,
    diameter: Option<Diameter>,
    teeth: Option<ToothCount>,
}
