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

        let (mode, start, target, planning) = match input {

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

                // planregel
                p.domain("cuts", FinishingPlanner::validate_cuts(cuts));

                if let (Some(s), Some(t)) = (start.clone(), target.clone()) {
                    p.domain(
                        "target_diameter_mm",
                        FinishingPlanner::validate_direction(mode, s, t),
                    );
                }

                (
                    mode,
                    start,
                    target,
                    Some(FinishingPlanning::ByCuts(cuts)),
                )
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
                    p.value(
                        "radial_engagement_mm",
                        PositiveLength::mm(radial_engagement_mm),
                    );

                if let Some(ae) = radial {
                    p.domain(
                        "radial_engagement_mm",
                        FinishingPlanner::validate_radial_engagement(ae),
                    );
                }

                if let (Some(s), Some(t)) = (start.clone(), target.clone()) {
                    p.domain(
                        "target_diameter_mm",
                        FinishingPlanner::validate_direction(mode, s, t),
                    );
                }

                (
                    mode,
                    start,
                    target,
                    radial.map(FinishingPlanning::ByRadialEngagement),
                )
            }
        };

        // -----------------------------------------------------
        // Build request if inputs are valid
        // -----------------------------------------------------

        let request = match (start, target, planning) {
            (Some(start), Some(target), Some(planning)) => Some(FinishingRequest {
                mode,
                start_diameter: start,
                target_diameter: target,
                planning,
            }),
            _ => None,
        };

        // -----------------------------------------------------
        // Domain planning
        // -----------------------------------------------------

        let plan = request.and_then(|req| {
            p.domain(
                "target_diameter_mm",
                FinishingPlanner::generate_plan(req),
            )
        });

        // -----------------------------------------------------
        // Create execution
        // -----------------------------------------------------

        let execution = plan.and_then(|plan| {

            let id = FinishingExecutionId::new();

            p.domain(
                "target_diameter_mm",
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