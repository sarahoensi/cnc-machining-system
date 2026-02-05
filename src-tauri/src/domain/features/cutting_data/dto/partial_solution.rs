// cutting_data/dto/partial_solution.rs

use crate::domain::features::cutting_data::model::values::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuttingDataPartialSolution {
    pub diameter: DiameterMm,
    pub teeth: ToothCount,

    pub cutting_speed: Option<CuttingSpeedMMin>,
    pub spindle_speed: Option<SpindleSpeedRpm>,
    pub feed_rate: Option<FeedRateMmMin>,
    pub feed_per_tooth: Option<FeedPerToothMm>,
}

