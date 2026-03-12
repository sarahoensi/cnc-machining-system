// application/cutting_data/solve_cutting_data_use_case.rs

use crate::application::shared::{AppResult, InputParser};

use crate::application::cutting_data::dto::{
    SolveCuttingDataInput,
    SolveCuttingDataOutput,
};

use crate::domain::{
    DomainError,
    machining::CuttingSolver,
    units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm, ToothCount},
};

pub struct SolveCuttingDataUseCase;

impl SolveCuttingDataUseCase {

    // ---------------------------------------------------------
    // Public boundary
    // ---------------------------------------------------------

    pub fn execute(
        input: SolveCuttingDataInput,
    ) -> AppResult<SolveCuttingDataOutput> {

        let mut p = InputParser::new();

        let parsed = ParsedCuttingInput {

            cutting_speed: p.optional(
                "cutting_speed_m_per_min",
                input.cutting_speed_m_per_min,
                CuttingSpeed::meters_per_min,
            ),

            rpm: p.optional(
                "rpm",
                input.rpm,
                Rpm::new,
            ),

            diameter: p.optional(
                "diameter_mm",
                input.diameter_mm,
                Diameter::mm,
            ),

            teeth: p.optional(
                "teeth",
                input.teeth,
                ToothCount::new,
            ),

            chip: p.optional(
                "chip_load_mm_per_tooth",
                input.chip_load_mm_per_tooth,
                ChipLoad::mm_per_tooth,
            ),

            feed: p.optional(
                "feed_rate_mm_per_min",
                input.feed_rate_mm_per_min,
                FeedRate::mm_per_min,
            ),
        };

        // stop early if parsing failed
        p.finish()?;

        let output = Self::solve(parsed)?;

        Ok(output)
    }

    // ---------------------------------------------------------
    // Domain orchestration
    // ---------------------------------------------------------

    fn solve(
        mut input: ParsedCuttingInput,
    ) -> Result<SolveCuttingDataOutput, DomainError> {

        let mut changed = true;

        while changed {
            changed = false;

            // vc ↔ rpm
            if input.rpm.is_none() {
                if let (Some(vc), Some(d)) = (input.cutting_speed, input.diameter) {
                    input.rpm =
                        Some(CuttingSolver::rpm_from_cutting_speed(vc, d)?);
                    changed = true;
                }
            }

            if input.cutting_speed.is_none() {
                if let (Some(n), Some(d)) = (input.rpm, input.diameter) {
                    input.cutting_speed =
                        Some(CuttingSolver::cutting_speed_from_rpm(n, d)?);
                    changed = true;
                }
            }

            // chip ↔ feed
            if input.feed.is_none() {
                if let (Some(fz), Some(n), Some(z)) =
                    (input.chip, input.rpm, input.teeth)
                {
                    input.feed =
                        Some(CuttingSolver::feed_from_chip_load(fz, n, z)?);
                    changed = true;
                }
            }

            if input.chip.is_none() {
                if let (Some(f), Some(n), Some(z)) =
                    (input.feed, input.rpm, input.teeth)
                {
                    input.chip =
                        Some(CuttingSolver::chip_from_feed(f, n, z)?);
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