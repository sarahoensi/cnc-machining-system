// solve.rs

use std::f64::consts::PI;

use super::input::{ValidCuttingInput, SpeedInput, FeedInput};
use super::solution::CuttingDataSolution;
use super::values::*;

/// Solves cutting data based on fully validated input.
///
/// This function assumes:
/// - all invariants already hold
/// - exactly one speed mode exists
/// - exactly one feed mode exists
///
/// No validation or error handling occurs here.
pub fn solve(input: ValidCuttingInput) -> CuttingDataSolution {
    let ValidCuttingInput {
        diameter,
        teeth,
        speed,
        feed,
    } = input;

    // --------------------------------------------------
    // Speed: Vc <-> n
    // --------------------------------------------------
    let (cutting_speed, spindle_speed) = match speed {
        SpeedInput::CuttingSpeed(vc) => {
            let n = spindle_speed_from_cutting_speed(vc, diameter);
            (vc, n)
        }
        SpeedInput::SpindleSpeed(n) => {
            let vc = cutting_speed_from_spindle_speed(n, diameter);
            (vc, n)
        }
    };

    // --------------------------------------------------
    // Feed: F <-> fz
    // --------------------------------------------------
    let (feed_rate, feed_per_tooth) = match feed {
        FeedInput::FeedRate(f) => {
            let fz = feed_per_tooth_from_feed_rate(f, teeth, spindle_speed);
            (f, fz)
        }
        FeedInput::FeedPerTooth(fz) => {
            let f = feed_rate_from_feed_per_tooth(fz, teeth, spindle_speed);
            (f, fz)
        }
    };

    CuttingDataSolution {
        diameter: diameter,
        teeth: teeth,
        cutting_speed,
        spindle_speed,
        feed_rate,
        feed_per_tooth,
    }
}

fn spindle_speed_from_cutting_speed(
    vc: CuttingSpeedMMin,
    d: DiameterMm,
) -> SpindleSpeedRpm {
    let n = (1000.0 * vc.value()) / (PI * d.value());
    SpindleSpeedRpm::new(n).expect("Derived spindle speed must be > 0")
}

fn cutting_speed_from_spindle_speed(
    n: SpindleSpeedRpm,
    d: DiameterMm,
) -> CuttingSpeedMMin {
    let vc = (PI * d.value() * n.value()) / 1000.0;
    CuttingSpeedMMin::new(vc).expect("Derived cutting speed must be > 0")
}

fn feed_rate_from_feed_per_tooth(
    fz: FeedPerToothMm,
    z: ToothCount,
    n: SpindleSpeedRpm,
) -> FeedRateMmMin {
    let f = fz.value() * z.value() as f64 * n.value();
    FeedRateMmMin::new(f).expect("Derived feed rate must be > 0")
}

fn feed_per_tooth_from_feed_rate(
    f: FeedRateMmMin,
    z: ToothCount,
    n: SpindleSpeedRpm,
) -> FeedPerToothMm {
    let fz = f.value() / (z.value() as f64 * n.value());
    FeedPerToothMm::new(fz).expect("Derived feed per tooth must be > 0")
}
