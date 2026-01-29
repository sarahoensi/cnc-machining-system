// solution.rs

use super::values::*;

/// Fully solved and consistent cutting data.
///
/// All values are guaranteed to be valid and expressed
/// in canonical units.
#[derive(Debug, Clone, PartialEq)]
pub struct CuttingDataSolution {
    pub diameter: DiameterMm,
    pub teeth: ToothCount,
    pub cutting_speed: CuttingSpeedMMin,
    pub spindle_speed: SpindleSpeedRpm,
    pub feed_rate: FeedRateMmMin,
    pub feed_per_tooth: FeedPerToothMm,
}
