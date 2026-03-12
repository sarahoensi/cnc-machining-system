use crate::application::shared::{AppResult, InputParser};

use crate::application::finishing::dto::GenerateFinishingPlanInput;

use crate::domain::{
    units::{Diameter, PositiveLength},
    machining::finishing::{
        FinishingExecution,
        FinishingPlanner,
        FinishingPlanning,
        FinishingRequest,
    },
};

pub struct GenerateFinishingPlanUseCase;

impl GenerateFinishingPlanUseCase {

    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        input: GenerateFinishingPlanInput,
    ) -> AppResult<FinishingExecution> {

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

        let request = match (start, target, planning) {
            (Some(start), Some(target), Some(planning)) => Some(FinishingRequest {
                mode,
                start_diameter: start,
                target_diameter: target,
                planning,
            }),
            _ => None,
        };

        let plan = request.and_then(|req| {
            p.domain(
                "target_diameter_mm",
                FinishingPlanner::generate_plan(req),
            )
        });

        let execution = plan.and_then(|plan| {
            p.domain(
                "target_diameter_mm",
                FinishingExecution::new(plan),
            )
        });

        let execution = p.finish_with(execution)?;

        Ok(execution)
    }
}