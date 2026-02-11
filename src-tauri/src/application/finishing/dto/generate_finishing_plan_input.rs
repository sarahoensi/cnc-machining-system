
use crate::domain::FinishingMode;

pub enum GenerateFinishingPlanInput {

    ByCuts {
        mode: FinishingMode,
        start_diameter_mm: f64,
        target_diameter_mm: f64,
        cuts: u32,
    },

    ByRadialEngagement {
        mode: FinishingMode,
        start_diameter_mm: f64,
        target_diameter_mm: f64,
        radial_engagement_mm: f64,
    },
}
