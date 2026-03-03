// domain/units/core/positive_scalar.rs

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PositiveScalar(f64);

impl PositiveScalar {
    pub fn new<E>(
        value: f64,
        not_finite: impl Fn(f64) -> E,
        non_positive: impl Fn(f64) -> E,
    ) -> Result<Self, E> {
        if !value.is_finite() {
            return Err(not_finite(value));
        }
        if value <= 0.0 {
            return Err(non_positive(value));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}