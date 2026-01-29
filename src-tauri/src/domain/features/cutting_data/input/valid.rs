// input.rs

use std::convert::TryFrom;

use super::raw_input::RawCuttingInput;
use super::values::*;
use super::errors::DomainError;

/// Represents the user-provided speed input.
/// Exactly one variant must exist in valid domain input.
#[derive(Debug, Clone, Copy)]
pub enum SpeedInput {
    CuttingSpeed(CuttingSpeedMMin),
    SpindleSpeed(SpindleSpeedRpm),
}

/// Represents the user-provided feed input.
/// Exactly one variant must exist in valid domain input.
#[derive(Debug, Clone, Copy)]
pub enum FeedInput {
    FeedRate(FeedRateMmMin),
    FeedPerTooth(FeedPerToothMm),
}

/// Fully validated cutting data input.
/// All domain invariants are guaranteed to hold.
#[derive(Debug, Clone)]
pub struct ValidCuttingInput {
    pub diameter: DiameterMm,
    pub teeth: ToothCount,
    pub speed: SpeedInput,
    pub feed: FeedInput,
}

impl TryFrom<RawCuttingInput> for ValidCuttingInput {
    type Error = DomainError;

    fn try_from(raw: RawCuttingInput) -> Result<Self, Self::Error> {
        // --------------------------------------------------
        // Required base fields
        // --------------------------------------------------
        let diameter = DiameterMm::new(
            raw.d.ok_or(DomainError::MissingField("D"))?
        )?;

        let teeth = ToothCount::new(
            raw.z.ok_or(DomainError::MissingField("z"))?
        )?;

        // --------------------------------------------------
        // Speed input mode (Vc XOR n)
        // --------------------------------------------------
        let speed = match (raw.vc, raw.n) {
            (Some(vc), None) => {
                SpeedInput::CuttingSpeed(
                    CuttingSpeedMMin::new(vc)?
                )
            }
            (None, Some(n)) => {
                SpeedInput::SpindleSpeed(
                    SpindleSpeedRpm::new(n)?
                )
            }
            _ => return Err(DomainError::InvalidSpeedMode),
        };

        // --------------------------------------------------
        // Feed input mode (F XOR fz)
        // --------------------------------------------------
        let feed = match (raw.f, raw.fz) {
            (Some(f), None) => {
                FeedInput::FeedRate(
                    FeedRateMmMin::new(f)?
                )
            }
            (None, Some(fz)) => {
                FeedInput::FeedPerTooth(
                    FeedPerToothMm::new(fz)?
                )
            }
            _ => return Err(DomainError::InvalidFeedMode),
        };

        Ok(Self {
            diameter,
            teeth,
            speed,
            feed,
        })
    }
}
