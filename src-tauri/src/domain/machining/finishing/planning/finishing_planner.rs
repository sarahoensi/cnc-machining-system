// domain/machining/finishing/planning/finishing_planner.rs

use crate::domain::{
    machining::finishing::{
    FinishingError, FinishingMode, FinishingPlan, FinishingPlanning, FinishingRequest},
    units::{Diameter, PositiveLength},
};

pub struct FinishingPlanner;

impl FinishingPlanner {
    pub fn generate_plan(req: FinishingRequest) -> Result<FinishingPlan, FinishingError> {
        let start = req.start_diameter.mm_value();
        let target = req.target_diameter.mm_value();

        if !req.mode.validate_direction(start, target) {
            return Err(FinishingError::InvalidModeDirection {
                start_mm: start,
                target_mm: target,
            });
        }

        let total_delta = (target - start).abs();

        if total_delta <= f64::EPSILON {
            return Err(FinishingError::DiametersMustDiffer);
        }

        let (cuts, expected_step) = match req.planning {
            FinishingPlanning::ByCuts(cuts) => {
                if cuts == 0 {
                    return Err(FinishingError::InvalidCutCount { cuts });
                }

                let step = total_delta / cuts as f64;
                let step = PositiveLength::mm(step)
                    .map_err(|_| FinishingError::ComputedStepNotPositive { value_mm: step })?;

                (cuts, step)
            }

            FinishingPlanning::ByRadialEngagement(ae) => {
                let ae_mm = ae.mm_value();

                if ae_mm <= 0.0 {
                    return Err(FinishingError::InvalidRadialEngagement { value_mm: ae_mm });
                }

                let delta_d = ae_mm * 2.0;
                let cuts = (total_delta / delta_d).ceil() as u32;

                if cuts == 0 {
                    return Err(FinishingError::ImpossiblePlan {
                        reason: "computed zero cuts from radial engagement",
                    });
                }

                let step = total_delta / cuts as f64;
                let step = PositiveLength::mm(step)
                    .map_err(|_| FinishingError::ComputedStepNotPositive { value_mm: step })?;

                (cuts, step)
            }
        };

        Ok(FinishingPlan::new(
            req.mode,
            req.start_diameter,
            req.target_diameter,
            cuts,
            expected_step,
        ))
    }


    pub fn validate_direction(
        mode: FinishingMode,
        start: Diameter,
        target: Diameter,
    ) -> Result<(), FinishingError> {

        let s = start.mm_value();
        let t = target.mm_value();

        match mode {
            FinishingMode::Inner if t <= s =>
                Err(FinishingError::InvalidModeDirection { start_mm: s, target_mm: t }),

            FinishingMode::Outer if t >= s =>
                Err(FinishingError::InvalidModeDirection { start_mm: s, target_mm: t }),

            _ => Ok(())
        }
    }

    pub fn validate_radial_engagement(
        ae: PositiveLength,
    ) -> Result<(), FinishingError> {

        if ae.mm_value() <= 0.0 {
            Err(FinishingError::InvalidRadialEngagement {
                value_mm: ae.mm_value()
            })
        } else {
            Ok(())
        }
    }

    pub fn validate_cuts(cuts: u32) -> Result<(), FinishingError> {

        if cuts == 0 {
            Err(FinishingError::InvalidCutCount { cuts })
        } else {
            Ok(())
        }
    }


}