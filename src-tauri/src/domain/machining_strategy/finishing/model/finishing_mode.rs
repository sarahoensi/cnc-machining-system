// domain/machining_strategy/finishing/finishing_mode.rs

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FinishingMode {
    /// Inner diameter finishing (typically increases diameter)
    Inner,
    /// Outer diameter finishing (typically decreases diameter)
    Outer,
}

impl FinishingMode {
    /// +1 for increasing diameter, -1 for decreasing diameter
    pub fn direction_sign(self) -> f64 {
        match self {
            FinishingMode::Inner => 1.0,
            FinishingMode::Outer => -1.0,
        }
    }
}
