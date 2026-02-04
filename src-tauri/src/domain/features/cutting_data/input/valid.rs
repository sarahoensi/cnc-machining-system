// cutting_data/input/valid.rs

use std::convert::TryFrom;

use crate::domain::features::cutting_data::model::*;
use crate::domain::features::cutting_data::errors::DomainError;
use super::raw::RawCuttingInput;

impl TryFrom<RawCuttingInput> for CuttingData {
    type Error = DomainError;

    fn try_from(raw: RawCuttingInput) -> Result<Self, Self::Error> {

        let diameter = DiameterMm::new(raw.d.ok_or(DomainError::MissingField("d"))?)?;
        let teeth = ToothCount::new(raw.z.ok_or(DomainError::MissingField("z"))?)?;

        // -------- SPEED --------
        let speed = match (raw.vc, raw.n) {
            (Some(vc), None) => {
                Some(Speed::CuttingSpeed(CuttingSpeedMMin::new(vc)?))
            }
            (None, Some(n)) => {
                Some(Speed::SpindleSpeed(SpindleSpeedRpm::new(n)?))
            }
            (None, None) => None,
            _ => return Err(DomainError::InvalidSpeedMode),
        };

        // -------- FEED --------
        let feed = match (raw.f, raw.fz) {
            (Some(f), None) => {
                Some(Feed::FeedRate(FeedRateMmMin::new(f)?))
            }
            (None, Some(fz)) => {
                Some(Feed::FeedPerTooth(FeedPerToothMm::new(fz)?))
            }
            (None, None) => None,
            _ => return Err(DomainError::InvalidFeedMode),
        };

        Ok(CuttingData {
            diameter,
            teeth,
            speed,
            feed,
        })
    }
}
