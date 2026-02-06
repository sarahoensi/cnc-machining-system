// domain/machining_physics/cutting_result.rs

use crate::domain::units::machining::{ChipLoad, CuttingSpeed};
use crate::domain::units::motion::{FeedRate, Rpm};

#[derive(Debug, Copy, Clone)]
pub struct CuttingResult {
    pub cutting_speed: CuttingSpeed,
    pub rpm: Rpm,
    pub chip_load: ChipLoad,
    pub feed_rate: FeedRate,
}
