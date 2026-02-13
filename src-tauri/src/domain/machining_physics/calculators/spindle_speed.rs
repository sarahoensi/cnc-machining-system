// domain/machining_physics/calculators/spindle_speed.rs

use std::f64::consts::PI;

use crate::domain::UnitError;
use crate::domain::{CuttingSpeed, Diameter, Rpm};


/// Provides conversions between spindle speed (RPM) and cutting speed.
///
/// # Machining Formulas
///
/// Cutting speed is defined as:
///
/// ```text
/// Vc = π * D * n / 1000
/// ```
///
/// Where:
///
/// - `Vc` = cutting speed (m/min)
/// - `D` = tool diameter (mm)
/// - `n` = spindle speed (RPM)
///
/// The factor `1000` converts millimeters to meters.
///
/// This service is stateless and purely computational.
pub struct SpindleSpeedCalculator;

impl SpindleSpeedCalculator {

    /// Calculates spindle speed from cutting speed and tool diameter.
    ///
    /// Formula:
    ///
    /// ```text
    /// n = (Vc * 1000) / (π * D)
    /// ```
    pub fn rpm_from_cutting_speed(
        cutting_speed: CuttingSpeed,
        diameter: Diameter,
    ) -> Result<Rpm, UnitError> {
        let d = diameter.mm_value();
        let vc = cutting_speed.meters_per_min_value();

        let rpm = (vc * 1000.0) / (PI * d);
        Rpm::new(rpm)
    }

    /// Calculates cutting speed from spindle speed and tool diameter.
    ///
    /// Formula:
    ///
    /// ```text
    /// Vc = π * D * n / 1000
    /// ```
    pub fn cutting_speed_from_rpm(
        rpm: Rpm,
        diameter: Diameter,
    ) -> Result<CuttingSpeed, UnitError> {
        let d = diameter.mm_value();
        let n = rpm.value();

        let vc = PI * d * n / 1000.0;
        CuttingSpeed::meters_per_min(vc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Diameter;

    #[test]
    fn roundtrip_rpm_vc() {
        let d = Diameter::mm(10.0).unwrap();
        let rpm = Rpm::new(1000.0).unwrap();

        let vc = SpindleSpeedCalculator::cutting_speed_from_rpm(rpm, d).unwrap();
        let rpm2 = SpindleSpeedCalculator::rpm_from_cutting_speed(vc, d).unwrap();

        assert!((rpm2.value() - 1000.0).abs() < 1e-6);
    }
}
