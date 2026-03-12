// domain/machining/finishing/finishing_mode.rs

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FinishingMode {
    Inner,
    Outer,
}

impl FinishingMode {
    pub fn apply_delta(self, base: f64, delta: f64) -> f64 {
        match self {
            Self::Inner => base + delta,
            Self::Outer => base - delta,
        }
    }

    pub fn progresses_forward(self, previous: f64, next: f64, eps: f64) -> bool {
        match self {
            Self::Inner => next + eps >= previous,
            Self::Outer => next - eps <= previous,
        }
    }

    pub fn within_bounds(self, start: f64, target: f64, value: f64, eps: f64) -> bool {
        match self {
            Self::Inner => value >= start - eps && value <= target + eps,
            Self::Outer => value <= start + eps && value >= target - eps,
        }
    }

    pub fn passes_target(self, target: f64, value: f64, eps: f64) -> bool {
        match self {
            Self::Inner => value > target + eps,
            Self::Outer => value < target - eps,
        }
    }

    pub fn validate_direction(self, start: f64, target: f64) -> bool {
        match self {
            Self::Inner => target > start,
            Self::Outer => target < start,
        }
    }
}