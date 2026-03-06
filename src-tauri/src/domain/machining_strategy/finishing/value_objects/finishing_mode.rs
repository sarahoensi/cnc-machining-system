// domain/machining_strategy/finishing/finishing_mode.rs

/// Defines the finishing direction relative to the machined surface.
///
/// Determines whether finishing operations target an inner or outer diameter,
/// which affects the direction of dimensional change.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FinishingMode {
    /// Inner diameter finishing.
    ///
    /// Typically removes material from an internal surface,
    /// resulting in an increased diameter.
    Inner,

    /// Outer diameter finishing.
    ///
    /// Typically removes material from an external surface,
    /// resulting in a decreased diameter.
    Outer,
}

impl FinishingMode {

    pub fn apply_delta(self, base: f64, delta: f64) -> f64 {
        match self {
            FinishingMode::Inner => base + delta,
            FinishingMode::Outer => base - delta,
        }
    }

    pub fn progresses_forward(
        self,
        previous: f64,
        next: f64,
        eps: f64,
    ) -> bool {
        match self {
            FinishingMode::Inner => next + eps >= previous,
            FinishingMode::Outer => next - eps <= previous,
        }
    }

    pub fn within_bounds(
        self,
        start: f64,
        target: f64,
        value: f64,
        eps: f64,
    ) -> bool {
        match self {
            FinishingMode::Inner =>
                value >= start - eps && value <= target + eps,

            FinishingMode::Outer =>
                value <= start + eps && value >= target - eps,
        }
    }

    pub fn passes_target(
        self,
        target: f64,
        value: f64,
        eps: f64,
    ) -> bool {
        match self {
            FinishingMode::Inner => value > target + eps,
            FinishingMode::Outer => value < target - eps,
        }
    }
}