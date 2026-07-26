use super::ThreadSpec;

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadResult {
    pub drill_diameter_mm: f64,
    pub thread_depth_mm: f64,
}

pub struct ThreadSolver;

impl ThreadSolver {
    pub fn solve(spec: &ThreadSpec) -> ThreadResult {
        ThreadResult {
            drill_diameter_mm: spec.tap_drill_mm,
            thread_depth_mm: spec.radial_thread_depth_mm,
        }
    }
}
