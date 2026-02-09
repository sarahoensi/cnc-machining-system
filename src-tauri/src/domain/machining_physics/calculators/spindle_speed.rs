// domain/machining_physics/calculators/spindle_speed.rs

use std::f64::consts::PI;

use crate::domain::units::errors::UnitError;
use crate::domain::units::Diameter;
use crate::domain::units::CuttingSpeed;
use crate::domain::units::Rpm;

pub struct SpindleSpeedCalculator;

impl SpindleSpeedCalculator {
    pub fn rpm_from_cutting_speed(
        cutting_speed: CuttingSpeed,
        diameter: Diameter,
    ) -> Result<Rpm, UnitError> {
        let d = diameter.mm_value();
        let vc = cutting_speed.meters_per_min_value();

        let rpm = (vc * 1000.0) / (PI * d);
        Rpm::new(rpm)
    }

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
    use crate::domain::units::Diameter;

    #[test]
    fn roundtrip_rpm_vc() {
        let d = Diameter::mm(10.0).unwrap();
        let rpm = Rpm::new(1000.0).unwrap();

        let vc = SpindleSpeedCalculator::cutting_speed_from_rpm(rpm, d).unwrap();
        let rpm2 = SpindleSpeedCalculator::rpm_from_cutting_speed(vc, d).unwrap();

        assert!((rpm2.value() - 1000.0).abs() < 1e-6);
    }
}
