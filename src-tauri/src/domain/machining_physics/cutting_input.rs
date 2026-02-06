// domain/machining_physics/cutting_input.rs

use crate::domain::units::machining::{ChipLoad, CuttingSpeed};
use crate::domain::units::motion::{FeedRate, Rpm};

/// Input for typical milling calculations.
/// Note: you often know two of these and calculate the third (rpm/feed/chipload).
#[derive(Debug, Copy, Clone)]
pub struct CuttingInput {
    pub cutting_speed: Option<CuttingSpeed>,
    pub rpm: Option<Rpm>,

    pub chip_load: Option<ChipLoad>,
    pub feed_rate: Option<FeedRate>,
}
