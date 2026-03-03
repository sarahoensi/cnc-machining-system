// domain/machining_physics/cutting_solver.rs

use crate::domain::{
     machining_physics::{
        CuttingParameters, MachiningPhysicsError, Tool,
    }, units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm, ToothCount}
};


use std::f64::consts::PI;

const EPS: f64 = 1e-12;
pub struct MachiningSolver;

impl MachiningSolver {

    // ---------------------------------------------------------
    // Cutting speed + chip load
    // ---------------------------------------------------------
    pub fn from_speed_and_chip_load(
    cutting_speed: CuttingSpeed,
    chip_load: ChipLoad,
    tool: Tool,
) -> Result<CuttingParameters, MachiningPhysicsError> {

    let rpm =
        Self::rpm_from_cutting_speed(
            cutting_speed,
            tool.diameter(),
        )?;

    let feed =
        Self::feed_from_chip_load(
            chip_load,
            rpm,
            tool.teeth(),
        )?;

    Ok(CuttingParameters::new(
        cutting_speed,
        rpm,
        chip_load,
        feed,
    ))
}

    // ---------------------------------------------------------
    // RPM + feed rate
    // ---------------------------------------------------------
    pub fn from_rpm_and_feed(
        rpm: Rpm,
        feed: FeedRate,
        tool: Tool,
    ) -> Result<CuttingParameters, MachiningPhysicsError> {

        let chip =
            Self::chip_from_feed(
                feed,
                rpm,
                tool.teeth(),
            )?;

        let cutting_speed =
            Self::cutting_speed_from_rpm(
                rpm,
                tool.diameter(),
            )?;

        Ok(CuttingParameters::new(
            cutting_speed,
            rpm,
            chip,
            feed,
        ))
    }

    pub fn rpm_from_cutting_speed(
    cutting_speed: CuttingSpeed,
    diameter: Diameter,
) -> Result<Rpm, MachiningPhysicsError> {

    // diameter er allerede > 0 og finite

    let rpm =
        (cutting_speed.meters_per_min_value() * 1000.0)
            / (PI * diameter.mm_value());

    if !rpm.is_finite() {
        return Err(MachiningPhysicsError::NumericalInstability);
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
}