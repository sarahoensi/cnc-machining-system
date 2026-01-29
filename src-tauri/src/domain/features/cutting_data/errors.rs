// errors.rs

#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    /// A required field was missing from raw input
    MissingField(&'static str),

    /// Speed input mode was invalid (both or neither provided)
    InvalidSpeedMode,

    /// Feed input mode was invalid (both or neither provided)
    InvalidFeedMode,

    /// A value failed validation (e.g. <= 0)
    InvalidValue(&'static str),
}
