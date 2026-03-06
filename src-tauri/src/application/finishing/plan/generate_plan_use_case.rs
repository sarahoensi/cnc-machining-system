// application/finishing/plan/generate_plan_use_case.rs

use std::sync::Arc;

use crate::application::{ApplicationError};
use crate::application::shared::{AppResult, InputParser};

use crate::application::finishing::dto::{
    FinishingExecutionOutput,
    GenerateFinishingPlanInput,
};

use crate::domain::{
    units::{Diameter, PositiveLength},
    FinishingExecution,
    FinishingExecutionId,
    FinishingExecutionRepository,
    FinishingPlanner,
    FinishingPlanning,
    FinishingRequest,
};

pub struct GenerateFinishingPlanUseCase {
    repo: Arc<dyn FinishingExecutionRepository>,
}

impl GenerateFinishingPlanUseCase {

    pub fn new(repo: Arc<dyn FinishingExecutionRepository>) -> Self {
        Self { repo }
    }

    pub fn execute(
        &self,
        input: GenerateFinishingPlanInput,
    ) -> AppResult<FinishingExecutionOutput> {

        let mut p = InputParser::new();

        // -----------------------------------------------------
        // Parse input → FinishingRequest
        // -----------------------------------------------------

        let request = match input {

            GenerateFinishingPlanInput::ByCuts {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                cuts,
            } => {

                let start =
                    p.value("start_diameter_mm", Diameter::mm(start_diameter_mm));

                let target =
                    p.value("target_diameter_mm", Diameter::mm(target_diameter_mm));

                if cuts == 0 {
                    p.push("cuts", "non_positive", "Cut count must be > 0");
                }

                match (start, target) {
                    (Some(start), Some(target)) => Some(FinishingRequest {
                        mode,
                        start_diameter: start,
                        target_diameter: target,
                        planning: FinishingPlanning::ByCuts(cuts),
                    }),
                    _ => None,
                }
            }

            GenerateFinishingPlanInput::ByRadialEngagement {
                mode,
                start_diameter_mm,
                target_diameter_mm,
                radial_engagement_mm,
            } => {

                let start =
                    p.value("start_diameter_mm", Diameter::mm(start_diameter_mm));

                let target =
                    p.value("target_diameter_mm", Diameter::mm(target_diameter_mm));

                let radial =
                    p.value("radial_engagement_mm", PositiveLength::mm(radial_engagement_mm));

                match (start, target, radial) {
                    (Some(start), Some(target), Some(radial)) => Some(FinishingRequest {
                        mode,
                        start_diameter: start,
                        target_diameter: target,
                        planning: FinishingPlanning::ByRadialEngagement(radial),
                    }),
                    _ => None,
                }
            }
        };

        // -----------------------------------------------------
        // Domain planning
        // -----------------------------------------------------

        let plan = request.and_then(|req|
            p.domain(
                "finishing_plan",
                FinishingPlanner::generate_plan(req),
            )
        );

        // -----------------------------------------------------
        // Create execution aggregate
        // -----------------------------------------------------

        let execution = plan.and_then(|plan| {

            let id = FinishingExecutionId::new();

            p.domain(
                "execution",
                FinishingExecution::new(id, plan),
            )
        });

        let execution = p.finish_with(execution)?;

        // -----------------------------------------------------
        // Persist
        // -----------------------------------------------------

        self.repo
            .save(execution.clone())
            .map_err(|e| ApplicationError::Infrastructure(e.to_string()))?;

        Ok((&execution).into())
    }
}