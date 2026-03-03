// application/cutting_data/solve_cutting_data_use_case.rs

use crate::application::{ApplicationError, ValidationErrors};
use crate::application::shared::AppResult;
use crate::application::cutting_data::dto::{SolveCuttingDataInput, SolveCuttingDataOutput};

use crate::domain::units::ToothCount;
use crate::domain::{
    units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm},
    MachiningPhysicsError,
    MachiningSolver,
};

pub struct SolveCuttingDataUseCase;

impl SolveCuttingDataUseCase {
    pub fn execute(input: SolveCuttingDataInput) -> AppResult<SolveCuttingDataOutput> {
        let mut v = ValidationErrors::new();

        let mut cutting_speed = parse_opt(
            "cutting_speed_m_per_min",
            input.cutting_speed_m_per_min,
            CuttingSpeed::meters_per_min,
            &mut v,
        );

        let mut rpm = parse_opt("rpm", input.rpm, Rpm::new, &mut v);

        let diameter = parse_opt("diameter_mm", input.diameter_mm, Diameter::mm, &mut v);

        let teeth = parse_opt("teeth", input.teeth, ToothCount::new, &mut v);

        let mut chip = parse_opt(
            "chip_load_mm_per_tooth",
            input.chip_load_mm_per_tooth,
            ChipLoad::mm_per_tooth,
            &mut v,
        );

        let mut feed = parse_opt(
            "feed_rate_mm_per_min",
            input.feed_rate_mm_per_min,
            FeedRate::mm_per_min,
            &mut v,
        );

        if !v.is_empty() {
            return Err(ApplicationError::Validation(v));
        }

        // -----------------------------------------------------
        // Forward-chaining inference loop
        // -----------------------------------------------------

        let mut changed = true;
        while changed {
            changed = false;

            // vc ↔ rpm (needs diameter)
            if rpm.is_none() {
                if let (Some(vc), Some(d)) = (cutting_speed, diameter) {
                    rpm = Some(MachiningSolver::rpm_from_cutting_speed(vc, d)
                        .map_err(map_machining_error)?);
                    changed = true;
                }
            }

            if cutting_speed.is_none() {
                if let (Some(n), Some(d)) = (rpm, diameter) {
                    cutting_speed = Some(MachiningSolver::cutting_speed_from_rpm(n, d)
                        .map_err(map_machining_error)?);
                    changed = true;
                }
            }

            // chip ↔ feed (needs rpm + teeth)
            if feed.is_none() {
                if let (Some(fz), Some(n), Some(z)) = (chip, rpm, teeth) {
                    feed = Some(MachiningSolver::feed_from_chip_load(fz, n, z)
                        .map_err(map_machining_error)?);
                    changed = true;
                }
            }

            if chip.is_none() {
                if let (Some(f), Some(n), Some(z)) = (feed, rpm, teeth) {
                    chip = Some(MachiningSolver::chip_from_feed(f, n, z)
                        .map_err(map_machining_error)?);
                    changed = true;
                }
            }
        }

        Ok(SolveCuttingDataOutput {
            cutting_speed_m_per_min: cutting_speed.map(|v| v.meters_per_min_value()),
            rpm: rpm.map(|v| v.value()),
            chip_load_mm_per_tooth: chip.map(|v| v.mm_per_tooth_value()),
            feed_rate_mm_per_min: feed.map(|v| v.mm_per_min_value()),
        })
    }
}

// ------------------------------------------------------------
// Helpers
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



fn map_machining_error(err: MachiningPhysicsError) -> ApplicationError {
    let mut v = ValidationErrors::new();

    

    ApplicationError::Validation(v)
}