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

        let (vc, rpm, chip, feed) = match input {

            SolveCuttingDataInput::FromCuttingSpeed {
                cutting_speed_m_per_min,
                diameter_mm,
                chip_load_mm_per_tooth,
                teeth,
            } => {
                let diameter = Diameter::mm(diameter_mm)?;
                let vc = CuttingSpeed::meters_per_min(cutting_speed_m_per_min)?;
                let chip = ChipLoad::mm_per_tooth(chip_load_mm_per_tooth)?;
                let teeth = ToothCount::new(teeth)?;

                let rpm = SpindleSpeedCalculator::rpm_from_cutting_speed(vc, diameter)?;
                let feed = FeedRateCalculator::feed_rate_from_chip_load(chip, rpm, teeth)?;

                (vc, rpm, chip, feed)
            }

            SolveCuttingDataInput::FromRpm {
                rpm,
                chip_load_mm_per_tooth,
                teeth,
                diameter_mm,
            } => {
                let rpm = Rpm::new(rpm)?;
                let chip = ChipLoad::mm_per_tooth(chip_load_mm_per_tooth)?;
                let teeth = ToothCount::new(teeth)?;
                let diameter = Diameter::mm(diameter_mm)?;

                let vc = SpindleSpeedCalculator::cutting_speed_from_rpm(rpm, diameter)?;
                let feed = FeedRateCalculator::feed_rate_from_chip_load(chip, rpm, teeth)?;

                (vc, rpm, chip, feed)
            }

            SolveCuttingDataInput::FromFeedRate {
                feed_rate_mm_per_min,
                rpm,
                teeth,
                diameter_mm,
            } => {
                let feed = FeedRate::mm_per_min(feed_rate_mm_per_min)?;
                let rpm = Rpm::new(rpm)?;
                let teeth = ToothCount::new(teeth)?;
                let diameter = Diameter::mm(diameter_mm)?;

                let chip = ChipLoadCalculator::chip_load_from_feed_rate(feed, rpm, teeth)?;
                let vc = SpindleSpeedCalculator::cutting_speed_from_rpm(rpm, diameter)?;

                (vc, rpm, chip, feed)
            }
        };

        Ok(SolveCuttingDataOutput {
            cutting_speed_m_per_min: vc.meters_per_min_value(),
            rpm: rpm.value(),
            chip_load_mm_per_tooth: chip.mm_per_tooth_value(),
            feed_rate_mm_per_min: feed.mm_per_min_value(),
        })
    }
}
