// cutting_data/dto/full_solution.rs

use crate::domain::features::cutting_data::model::values::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CuttingDataFullSolution {

    pub diameter: DiameterMm,
    pub teeth: ToothCount,
    pub cutting_speed: CuttingSpeedMMin,
    pub spindle_speed: SpindleSpeedRpm,
    pub feed_rate: FeedRateMmMin,
    pub feed_per_tooth: FeedPerToothMm,
}
