//domain/machining_physics/cutting_solver.rs

use crate::domain::{
    machining_physics::{
        CuttingParameters, MachiningPhysicsError, Tool,
    },
    units::{ChipLoad, CuttingSpeed, Diameter, FeedRate, Rpm, ToothCount},
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

    // ---------------------------------------------------------
    // Core physics
    // ---------------------------------------------------------

     /// n = (1000 * Vc) / (π * D)
    pub fn rpm_from_cutting_speed(
        cutting_speed: CuttingSpeed,
        diameter: Diameter,
    ) -> Result<Rpm, MachiningPhysicsError> {

        let rpm =
            (cutting_speed.meters_per_min_value() * 1000.0)
                / (PI * diameter.mm_value());

        if !rpm.is_finite() {
            return Err(MachiningPhysicsError::NumericalInstability);
        }

        Ok(Rpm::new(rpm)?)
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

        Ok(CuttingSpeed::meters_per_min(vc)?)
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

        if !f.is_finite() {
            return Err(MachiningPhysicsError::NumericalInstability);
        }

        Ok(FeedRate::mm_per_min(f)?)
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

        if !chip.is_finite() {
            return Err(MachiningPhysicsError::NumericalInstability);
        }

        Ok(ChipLoad::mm_per_tooth(chip)?)
    }
}

