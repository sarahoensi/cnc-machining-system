// application/cutting_data/solve_cutting_data_use_case.rs

use crate::application::{ApplicationError, ValidationErrors};
use crate::application::shared::AppResult;
use crate::application::cutting_data::dto::{
    SolveCuttingDataInput,
    SolveCuttingDataOutput,
};

use crate::domain::{
    DomainError,
    MachiningSolver,
    units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm, ToothCount},
};

pub struct SolveCuttingDataUseCase;

impl SolveCuttingDataUseCase {

    // ---------------------------------------------------------
    // Public boundary (Application layer)
    // ---------------------------------------------------------

    pub fn execute(input: SolveCuttingDataInput)
        -> AppResult<SolveCuttingDataOutput>
    {
        let mut v = ValidationErrors::new();

        let parsed = Self::parse_input(input, &mut v);

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        let parsed = parsed.unwrap(); // safe: we checked errors

        let output = Self::solve(parsed)?; // DomainError → ApplicationError

        Ok(output)
    }

    // ---------------------------------------------------------
    // Parsing (Application concern)
    // ---------------------------------------------------------

    fn parse_input(
        input: SolveCuttingDataInput,
        v: &mut ValidationErrors,
    ) -> Option<ParsedCuttingInput> {

        let cutting_speed = parse_opt(
            "cutting_speed_m_per_min",
            input.cutting_speed_m_per_min,
            CuttingSpeed::meters_per_min,
            v,
        );

        let rpm = parse_opt("rpm", input.rpm, Rpm::new, v);

        let diameter = parse_opt("diameter_mm", input.diameter_mm, Diameter::mm, v);

        let teeth = parse_opt("teeth", input.teeth, ToothCount::new, v);

        let chip = parse_opt(
            "chip_load_mm_per_tooth",
            input.chip_load_mm_per_tooth,
            ChipLoad::mm_per_tooth,
            v,
        );

        let feed = parse_opt(
            "feed_rate_mm_per_min",
            input.feed_rate_mm_per_min,
            FeedRate::mm_per_min,
            v,
        );

        Some(ParsedCuttingInput {
            cutting_speed,
            rpm,
            diameter,
            teeth,
            chip,
            feed,
        })
    }

    // ---------------------------------------------------------
    // Domain orchestration (pure domain errors)
    // ---------------------------------------------------------

    fn solve(
        mut input: ParsedCuttingInput,
    ) -> Result<SolveCuttingDataOutput, DomainError> {

        let mut changed = true;

        while changed {
            changed = false;

            // vc ↔ rpm (needs diameter)
            if input.rpm.is_none() {
                if let (Some(vc), Some(d)) = (input.cutting_speed, input.diameter) {
                    input.rpm =
                        Some(MachiningSolver::rpm_from_cutting_speed(vc, d)?);
                    changed = true;
                }
            }

            if input.cutting_speed.is_none() {
                if let (Some(n), Some(d)) = (input.rpm, input.diameter) {
                    input.cutting_speed =
                        Some(MachiningSolver::cutting_speed_from_rpm(n, d)?);
                    changed = true;
                }
            }

            // chip ↔ feed (needs rpm + teeth)
            if input.feed.is_none() {
                if let (Some(fz), Some(n), Some(z)) =
                    (input.chip, input.rpm, input.teeth)
                {
                    input.feed =
                        Some(MachiningSolver::feed_from_chip_load(fz, n, z)?);
                    changed = true;
                }
            }

            if input.chip.is_none() {
                if let (Some(f), Some(n), Some(z)) =
                    (input.feed, input.rpm, input.teeth)
                {
                    input.chip =
                        Some(MachiningSolver::chip_from_feed(f, n, z)?);
                    changed = true;
                }
            }
        }

        Ok(SolveCuttingDataOutput {
            cutting_speed_m_per_min:
                input.cutting_speed.map(|v| v.meters_per_min_value()),
            rpm:
                input.rpm.map(|v| v.value()),
            chip_load_mm_per_tooth:
                input.chip.map(|v| v.mm_per_tooth_value()),
            feed_rate_mm_per_min:
                input.feed.map(|v| v.mm_per_min_value()),
        })
    }
}

// ------------------------------------------------------------
// Internal parsed representation
// ------------------------------------------------------------

struct ParsedCuttingInput {
    cutting_speed: Option<CuttingSpeed>,
    rpm: Option<Rpm>,
    diameter: Option<Diameter>,
    teeth: Option<ToothCount>,
    chip: Option<ChipLoad>,
    feed: Option<FeedRate>,
}

// ------------------------------------------------------------
// Shared parsing helper
// ------------------------------------------------------------

fn parse_opt<T, Raw, F, E>(
    field: &'static str,
    raw: Option<Raw>,
    ctor: F,
    v: &mut ValidationErrors,
) -> Option<T>
where
    F: FnOnce(Raw) -> Result<T, E>,
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