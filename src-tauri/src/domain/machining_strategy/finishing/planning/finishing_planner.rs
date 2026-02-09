// domain/machining_strategy/finishing/finishing_planner.rs

use crate::domain::machining_strategy::strategy_error::StrategyError;
use crate::domain::{
    FinishingMode,
    FinishingPlan,
    FinishingPlanning,
    FinishingRequest,
    Length,
};


/// Domain service responsible for generating a valid FinishingPlan.
///
/// Supports multiple planning strategies:
/// - By number of cuts
/// - By radial engagement (ae)
pub struct FinishingPlanner;

impl FinishingPlanner {

    /// Generate a static finishing plan.
    pub fn generate_plan(req: FinishingRequest) -> Result<FinishingPlan, StrategyError> {

        let start = req.start_diameter.mm_value();
        let target = req.target_diameter.mm_value();

        // ------------------------------------------------------------
        // Validate direction vs mode
        // ------------------------------------------------------------
        match req.mode {
            FinishingMode::Inner => {
                if target <= start {
                    return Err(StrategyError::InvalidInputs(
                        "Inner mode requires target_diameter > start_diameter",
                    ));
                }
            }

            FinishingMode::Outer => {
                if target >= start {
                    return Err(StrategyError::InvalidInputs(
                        "Outer mode requires target_diameter < start_diameter",
                    ));
                }
            }
        }

        // ------------------------------------------------------------
        // Calculate total diameter delta
        // ------------------------------------------------------------
        let total_delta = (target - start).abs();

        if total_delta <= f64::EPSILON {
            return Err(StrategyError::InvalidInputs(
                "start_diameter and target_diameter must differ",
            ));
        }

        // ------------------------------------------------------------
        // Determine cuts + expected step depending on planning strategy
        // ------------------------------------------------------------
        let (cuts, expected_step) = match req.planning {

            // --------------------------------------------------------
            // Strategy 1: User specifies number of cuts
            // --------------------------------------------------------
            FinishingPlanning::ByCuts(cuts) => {

                if cuts == 0 {
                    return Err(StrategyError::InvalidInputs(
                        "cuts must be >= 1",
                    ));
                }

                let step_mag = total_delta / cuts as f64;

                let step = Length::mm_positive(step_mag)
                    .map_err(|_| StrategyError::InvalidInputs(
                        "computed step must be > 0",
                    ))?;

                (cuts, step)
            }

            // --------------------------------------------------------
            // Strategy 2: User specifies radial engagement (ae)
            // --------------------------------------------------------
            FinishingPlanning::ByRadialEngagement(ae) => {

                let ae_mm = ae.mm_value();

                if ae_mm <= 0.0 {
                    return Err(StrategyError::InvalidInputs(
                        "radial engagement (ae) must be > 0",
                    ));
                }

                // Convert radial engagement -> diameter delta
                let delta_d = ae_mm * 2.0;

                // Determine number of cuts (ceil ensures we reach target)
                let cuts = (total_delta / delta_d).ceil() as u32;

                if cuts == 0 {
                    return Err(StrategyError::ImpossiblePlan(
                        "computed zero cuts from radial engagement",
                    ));
                }

                // Recalculate exact step so final step hits target exactly
                let step_mag = total_delta / cuts as f64;

                let step = Length::mm_positive(step_mag)
                    .map_err(|_| StrategyError::InvalidInputs(
                        "computed step must be > 0",
                    ))?;

                (cuts, step)
            }
        };

        // ------------------------------------------------------------
        // Construct finishing plan
        // ------------------------------------------------------------
        Ok(FinishingPlan::new(
            req.mode,
            req.start_diameter,
            req.target_diameter,
            cuts,
            expected_step,
        ))
    }
}
