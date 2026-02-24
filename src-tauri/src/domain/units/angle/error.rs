// units/angle/error.rs

#[derive(Debug, Clone, PartialEq)]
pub enum AngleError {
    NotFinite {
        value: f64,
    },

    NotAcute {
        degrees: f64,
    },
}

use std::fmt;

impl fmt::Display for AngleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AngleError {}