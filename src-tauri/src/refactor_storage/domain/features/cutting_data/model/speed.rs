// cutting_data/model/speed.rs

use crate::domain::features::cutting_data::model::values::*;

#[derive(Debug, Clone, Copy)]
pub enum Speed {
    CuttingSpeed(CuttingSpeedMMin),
    SpindleSpeed(SpindleSpeedRpm),
}
