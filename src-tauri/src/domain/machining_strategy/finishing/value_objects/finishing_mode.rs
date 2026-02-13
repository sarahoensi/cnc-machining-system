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
    /// Returns the dimensional change direction.
    ///
    /// Returns:
    /// - `+1.0` for increasing diameter
    /// - `-1.0` for decreasing diameter
    ///
    /// This is commonly used when applying diameter deltas in finishing calculations.
    pub fn direction_sign(self) -> f64 {
        match self {
            FinishingMode::Inner => 1.0,
            FinishingMode::Outer => -1.0,
        }
    }
}
