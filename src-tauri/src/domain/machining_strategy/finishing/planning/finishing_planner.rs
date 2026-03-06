// domain/machining_strategy/finishing/finishing_planner.rs


use crate::domain::{
    FinishingMode, FinishingPlan, FinishingPlanning, FinishingRequest, machining_strategy::strategy_error::StrategyError, units::{PositiveLength}
};

/// Domain service responsible for generating a valid [`FinishingPlan`].
///
/// This service interprets machining intent from [`FinishingRequest`] and
/// converts it into a deterministic finishing strategy.
///
/// Supported planning strategies:
/// - Planning by explicit number of cuts
/// - Planning by radial engagement (ae)
pub struct FinishingPlanner;

impl FinishingPlanner {
    /// Generates a static finishing plan.
    ///
    /// The planner validates input consistency, determines the number of cuts,
    /// and calculates the expected diameter change per step.
    ///
    /// # Validation Rules
    ///
    /// - Inner finishing requires `target_diameter > start_diameter`
    /// - Outer finishing requires `target_diameter < start_diameter`
    /// - Start and target diameters must differ
    /// - Planning parameters must produce at least one valid cut
    ///
    /// # Planning Strategies
    ///
    /// ## ByCuts
    /// Uses the provided number of cuts and distributes the total diameter
    /// delta evenly across all passes.
    ///
    /// ## ByRadialEngagement
    /// Converts radial engagement (ae) to diameter delta and determines
    /// the minimum number of cuts required to reach the target diameter.
    pub fn generate_plan(req: FinishingRequest) -> Result<FinishingPlan, StrategyError> {
        let start = req.start_diameter.mm_value();
        let target = req.target_diameter.mm_value();

        // ------------------------------------------------------------
        // Validate direction vs mode
        // ------------------------------------------------------------
        match req.mode {
            FinishingMode::Inner => {
                if target <= start {
                    return Err(StrategyError::InvalidModeDirection {
                        start_mm: start,
                        target_mm: target,
                    });
                }
            }
            FinishingMode::Outer => {
                if target >= start {
                    return Err(StrategyError::InvalidModeDirection {
                        start_mm: start,
                        target_mm: target,
                    });
                }
            }
        }

        // ------------------------------------------------------------
        // Calculate total diameter delta
        // ------------------------------------------------------------
        let total_delta = (target - start).abs();

        if total_delta <= f64::EPSILON {
            return Err(StrategyError::DiametersMustDiffer);
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
                    return Err(StrategyError::InvalidCutCount { cuts });
                }

                let step_mag = total_delta / cuts as f64;

                let step = PositiveLength::mm(step_mag)
                    .map_err(|_| StrategyError::ComputedStepNotPositive {
                        value_mm: step_mag,
                    })?;

                (cuts, step)
            }

            // --------------------------------------------------------
            // Strategy 2: User specifies radial engagement (ae)
            // --------------------------------------------------------
            FinishingPlanning::ByRadialEngagement(ae) => {
                let ae_mm = ae.mm_value();

                if ae_mm <= 0.0 {
                    return Err(StrategyError::InvalidRadialEngagement {
                        value_mm: ae_mm,
                    });
                }

                // Convert radial engagement -> diameter delta
                let delta_d = ae_mm * 2.0;

                // Determine number of cuts (ceil ensures we reach target)
                let cuts = (total_delta / delta_d).ceil() as u32;

                if cuts == 0 {
                    return Err(StrategyError::ImpossiblePlan {
                        reason: "computed zero cuts from radial engagement",
                    });
                }

                // Recalculate exact step so final step hits target exactly
                let step_mag = total_delta / cuts as f64;

                let step = PositiveLength::mm(step_mag)
                    .map_err(|_| StrategyError::ComputedStepNotPositive {
                        value_mm: step_mag,
                    })?;

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
