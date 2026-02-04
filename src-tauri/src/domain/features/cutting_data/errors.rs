// cutting_data/errors.rs

#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    MissingField(&'static str),
    InvalidSpeedMode,
    InvalidFeedMode,
    InvalidValue(&'static str),
}
