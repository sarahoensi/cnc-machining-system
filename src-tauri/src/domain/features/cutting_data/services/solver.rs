// cutting_data/services/solver.rs

use crate::domain::features::cutting_data::model::*;
use crate::domain::features::cutting_data::dto::*;
use crate::domain::features::cutting_data::errors::DomainError;

use std::f64::consts::PI;

pub struct CuttingDataSolver;

impl CuttingDataSolver {

    // ---------------- PARTIAL SOLVER ----------------

    pub fn solve_partial(
        data: &CuttingData,
    ) -> Result<CuttingDataPartialSolution, DomainError> {

        // ---------- SPEED ----------
        let (cutting_speed, spindle_speed) = match data.speed {
            Some(Speed::CuttingSpeed(vc)) => {
                let n = Self::spindle_from_vc(vc, data.diameter)?;
                (Some(vc), Some(n))
            }

            Some(Speed::SpindleSpeed(n)) => {
                let vc = Self::vc_from_spindle(n, data.diameter)?;
                (Some(vc), Some(n))
            }

            None => (None, None),
        };

        // ---------- FEED ----------
        let (feed_rate, feed_per_tooth) = match (data.feed, spindle_speed) {

            (Some(Feed::FeedRate(f)), Some(n)) => {
                let fz = Self::fz_from_feed(f, data.teeth, n)?;
                (Some(f), Some(fz))
            }

            (Some(Feed::FeedPerTooth(fz)), Some(n)) => {
                let f = Self::feed_from_fz(fz, data.teeth, n)?;
                (Some(f), Some(fz))
            }

            _ => (None, None),
        };

        Ok(CuttingDataPartialSolution {
            diameter: data.diameter,
            teeth: data.teeth,
            cutting_speed,
            spindle_speed,
            feed_rate,
            feed_per_tooth,
        })
    }

    // ---------------- FULL SOLVER ----------------

    pub fn solve_full(
        data: &CuttingData,
    ) -> Result<CuttingDataFullSolution, DomainError> {

        let partial = Self::solve_partial(&data)?;

        Ok(CuttingDataFullSolution {
            diameter: data.diameter,
            teeth: data.teeth,
            cutting_speed: partial
                .cutting_speed
                .ok_or(DomainError::MissingField("cutting_speed"))?,
            spindle_speed: partial
                .spindle_speed
                .ok_or(DomainError::MissingField("spindle_speed"))?,
            feed_rate: partial
                .feed_rate
                .ok_or(DomainError::MissingField("feed_rate"))?,
            feed_per_tooth: partial
                .feed_per_tooth
                .ok_or(DomainError::MissingField("feed_per_tooth"))?,
        })
    }

    // ---------------- FORMULAS ----------------

    pub fn spindle_from_vc(
        vc: CuttingSpeedMMin,
        d: DiameterMm,
    ) -> Result<SpindleSpeedRpm, DomainError> {

        SpindleSpeedRpm::new((1000.0 * vc.value()) / (PI * d.value()))
    }

    pub fn vc_from_spindle(
        n: SpindleSpeedRpm,
        d: DiameterMm,
    ) -> Result<CuttingSpeedMMin, DomainError> {

        CuttingSpeedMMin::new((PI * d.value() * n.value()) / 1000.0)
    }

    pub fn feed_from_fz(
        fz: FeedPerToothMm,
        z: ToothCount,
        n: SpindleSpeedRpm,
    ) -> Result<FeedRateMmMin, DomainError> {

        FeedRateMmMin::new(fz.value() * z.value() as f64 * n.value())
    }

    pub fn fz_from_feed(
        f: FeedRateMmMin,
        z: ToothCount,
        n: SpindleSpeedRpm,
    ) -> Result<FeedPerToothMm, DomainError> {

        FeedPerToothMm::new(f.value() / (z.value() as f64 * n.value()))
    }
}


