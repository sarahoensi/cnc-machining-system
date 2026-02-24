// domain/machining_physics/formulas.rs

use std::f64::consts::PI;

use crate::domain::{
    machining_physics::tool::ToothCount,
    units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm},
};

use super::MachiningPhysicsError;

const EPS: f64 = 1e-12;

/// n = (Vc * 1000) / (π * D)
pub fn rpm_from_cutting_speed(
    cutting_speed: CuttingSpeed,
    diameter: Diameter,
) -> Result<Rpm, MachiningPhysicsError> {

    let d = diameter.mm_value();

    if d <= 0.0 || !d.is_finite() {
        return Err(MachiningPhysicsError::InvalidDiameter {
            value_mm: d,
        });
    }

    let rpm =
        (cutting_speed.meters_per_min_value() * 1000.0)
            / (PI * d);

    if rpm <= 0.0 || !rpm.is_finite() {
        return Err(MachiningPhysicsError::InvalidRpm {
            value: rpm,
        });
    }

    Rpm::new(rpm)
        .map_err(|_| MachiningPhysicsError::InvalidRpm { value: rpm })
}

/// Vc = π * D * n / 1000
pub fn cutting_speed_from_rpm(
    rpm: Rpm,
    diameter: Diameter,
) -> Result<CuttingSpeed, MachiningPhysicsError> {

    let vc =
        PI * diameter.mm_value() * rpm.value() / 1000.0;

    if !vc.is_finite() {
        return Err(MachiningPhysicsError::NumericalInstability);
    }

    CuttingSpeed::meters_per_min(vc)
        .map_err(|_| MachiningPhysicsError::NumericalInstability)
}

/// F = fz * n * z
pub fn feed_from_chip_load(
    chip: ChipLoad,
    rpm: Rpm,
    teeth: ToothCount,
) -> Result<FeedRate, MachiningPhysicsError> {

    let f =
        chip.mm_per_tooth_value()
            * rpm.value()
            * f64::from(teeth.value());

    if f <= 0.0 || !f.is_finite() {
        return Err(MachiningPhysicsError::InvalidFeedRate {
            value: f,
        });
    }

    FeedRate::mm_per_min(f)
        .map_err(|_| MachiningPhysicsError::InvalidFeedRate { value: f })
}

/// fz = F / (n * z)
pub fn chip_from_feed(
    feed: FeedRate,
    rpm: Rpm,
    teeth: ToothCount,
) -> Result<ChipLoad, MachiningPhysicsError> {

    let denom =
        rpm.value() * f64::from(teeth.value());

    if denom.abs() < EPS {
        return Err(MachiningPhysicsError::DivisionByZero);
    }

    let chip =
        feed.mm_per_min_value() / denom;

    if chip <= 0.0 || !chip.is_finite() {
        return Err(MachiningPhysicsError::InvalidChipLoad {
            value: chip,
        });
    }

    ChipLoad::mm_per_tooth(chip)
        .map_err(|_| MachiningPhysicsError::InvalidChipLoad { value: chip })
}