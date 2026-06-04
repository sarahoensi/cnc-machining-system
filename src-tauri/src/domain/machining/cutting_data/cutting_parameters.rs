// domain/machining/cutting_data/cutting_parameters.rs

use crate::domain::units::{ChipLoad, CuttingSpeed, FeedRate, Rpm};

/// Represents a mathematically consistent set of machining parameters.
///
/// Invariants:
/// - cutting_speed ↔ rpm ↔ diameter
/// - feed_rate ↔ chip_load ↔ rpm ↔ tooth_count
#[derive(Debug, Copy, Clone)]
pub struct CuttingParameters {
    cutting_speed: CuttingSpeed,
    rpm: Rpm,
    chip_load: ChipLoad,
    feed_rate: FeedRate,
}

impl CuttingParameters {
    pub(crate) fn new(
        cutting_speed: CuttingSpeed,
        rpm: Rpm,
        chip_load: ChipLoad,
        feed_rate: FeedRate,
    ) -> Self {
        Self {
            cutting_speed,
            rpm,
            chip_load,
            feed_rate,
        }
    }

    pub fn cutting_speed(&self) -> CuttingSpeed {
        self.cutting_speed
    }

    pub fn rpm(&self) -> Rpm {
        self.rpm
    }

    pub fn chip_load(&self) -> ChipLoad {
        self.chip_load
    }

    pub fn feed_rate(&self) -> FeedRate {
        self.feed_rate
    }
}
