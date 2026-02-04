//cutting_data/model/cutting_data.rs

use super::speed::Speed;
use super::feed::Feed;
use super::values::*;

#[derive(Debug, Clone)]
pub struct CuttingData {
    pub diameter: DiameterMm,
    pub teeth: ToothCount,
    pub speed: Option<Speed>,
    pub feed: Option<Feed>,
}
