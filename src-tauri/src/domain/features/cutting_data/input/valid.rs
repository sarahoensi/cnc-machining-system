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


// TESTER
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::features::cutting_data::errors::DomainError;

    #[test]
    fn missing_d_returns_error() {
        let raw = RawCuttingInput { d: None, z: Some(4), vc: Some(200.0), ..Default::default() };
        let err = CuttingData::try_from(raw).unwrap_err();
        assert_eq!(err, DomainError::MissingField("d"));
    }

    #[test]
    fn missing_z_returns_error() {
        let raw = RawCuttingInput { d: Some(10.0), z: None, vc: Some(200.0), ..Default::default() };
        let err = CuttingData::try_from(raw).unwrap_err();
        assert_eq!(err, DomainError::MissingField("z"));
    }

    #[test]
    fn speed_mode_invalid_if_both_vc_and_n() {
        let raw = RawCuttingInput {
            d: Some(10.0),
            z: Some(4),
            vc: Some(200.0),
            n: Some(6000.0),
            ..Default::default()
        };
        let err = CuttingData::try_from(raw).unwrap_err();
        assert_eq!(err, DomainError::InvalidSpeedMode);
    }

    #[test]
    fn speed_mode_allows_none_none_in_partial() {
        let raw = RawCuttingInput { d: Some(10.0), z: Some(4), vc: None, n: None, ..Default::default() };
        let data = CuttingData::try_from(raw).unwrap();
        assert!(data.speed.is_none());
    }

    #[test]
    fn feed_mode_invalid_if_both_f_and_fz() {
        let raw = RawCuttingInput {
            d: Some(10.0),
            z: Some(4),
            vc: Some(200.0),
            f: Some(1000.0),
            fz: Some(0.05),
            ..Default::default()
        };
        let err = CuttingData::try_from(raw).unwrap_err();
        assert_eq!(err, DomainError::InvalidFeedMode);
    }

    #[test]
    fn feed_mode_allows_none_none_in_partial() {
        let raw = RawCuttingInput { d: Some(10.0), z: Some(4), vc: Some(200.0), f: None, fz: None, ..Default::default() };
        let data = CuttingData::try_from(raw).unwrap();
        assert!(data.feed.is_none());
    }

    #[test]
    fn invalid_values_are_rejected() {
        let raw = RawCuttingInput { d: Some(0.0), z: Some(4), vc: Some(200.0), ..Default::default() };
        let err = CuttingData::try_from(raw).unwrap_err();
        match err {
            DomainError::InvalidValue(_) => {}
            other => panic!("Expected InvalidValue, got {:?}", other),
        }
    }
}


