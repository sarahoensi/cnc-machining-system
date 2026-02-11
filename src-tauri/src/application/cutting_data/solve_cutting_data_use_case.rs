// application/cutting_data/solve_cutting_data_use_case.rs

use crate::application::shared::AppResult;

use crate::application::cutting_data::dto::{
    SolveCuttingDataInput,
    SolveCuttingDataOutput,
};

use crate::domain::{
    ChipLoad,
    CuttingSpeed,
    Diameter,
    FeedRate,
    Rpm,
    ToothCount,
};

use crate::domain::{
    ChipLoadCalculator,
    FeedRateCalculator,
    SpindleSpeedCalculator,
};

pub struct SolveCuttingDataUseCase;

impl SolveCuttingDataUseCase {

    pub fn execute(
        input: SolveCuttingDataInput,
    ) -> AppResult<SolveCuttingDataOutput> {

        // --- Convert input to value objects if present ---
        let mut vc = input
            .cutting_speed_m_per_min
            .map(CuttingSpeed::meters_per_min)
            .transpose()?;

        let mut rpm = input
            .rpm
            .map(Rpm::new)
            .transpose()?;

        let mut chip = input
            .chip_load_mm_per_tooth
            .map(ChipLoad::mm_per_tooth)
            .transpose()?;

        let mut feed = input
            .feed_rate_mm_per_min
            .map(FeedRate::mm_per_min)
            .transpose()?;

        let diameter = input
            .diameter_mm
            .map(Diameter::mm)
            .transpose()?;

        let teeth = input
            .teeth
            .map(ToothCount::new)
            .transpose()?;

        // --- Forward chaining loop ---
        let mut changed = true;

        while changed {
            changed = false;

            // VC + Diameter → RPM
            if rpm.is_none() {
                if let (Some(vc_val), Some(d)) = (vc, diameter) {
                    rpm = Some(SpindleSpeedCalculator::rpm_from_cutting_speed(vc_val, d)?);
                    changed = true;
                }
            }

            // RPM + Diameter → VC
            if vc.is_none() {
                if let (Some(rpm_val), Some(d)) = (rpm, diameter) {
                    vc = Some(SpindleSpeedCalculator::cutting_speed_from_rpm(rpm_val, d)?);
                    changed = true;
                }
            }

            // Chip + RPM + Teeth → Feed
            if feed.is_none() {
                if let (Some(chip_val), Some(rpm_val), Some(t)) = (chip, rpm, teeth) {
                    feed = Some(FeedRateCalculator::feed_rate_from_chip_load(chip_val, rpm_val, t)?);
                    changed = true;
                }
            }

            // Feed + RPM + Teeth → Chip
            if chip.is_none() {
                if let (Some(feed_val), Some(rpm_val), Some(t)) = (feed, rpm, teeth) {
                    chip = Some(ChipLoadCalculator::chip_load_from_feed_rate(feed_val, rpm_val, t)?);
                    changed = true;
                }
            }
        }

        Ok(SolveCuttingDataOutput {
            cutting_speed_m_per_min: vc.map(|v| v.meters_per_min_value()),
            rpm: rpm.map(|r| r.value()),
            chip_load_mm_per_tooth: chip.map(|c| c.mm_per_tooth_value()),
            feed_rate_mm_per_min: feed.map(|f| f.mm_per_min_value()),
        })
    }
}
