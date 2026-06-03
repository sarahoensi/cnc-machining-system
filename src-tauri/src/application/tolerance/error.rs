use thiserror::Error;

#[derive(Debug, Error)]
pub enum Iso286Error {
    #[error("Nominal size must be greater than zero")]
    InvalidNominalSize,

    #[error("Invalid tolerance code '{0}'. Use letters followed by digits, for example H7, JS7, g6, or js6")]
    InvalidToleranceCode(String),

    #[error("Expected feature to be 'hole' or 'shaft', got '{0}'")]
    InvalidFeature(String),

    #[error("Tolerance class '{code}' is not supported for {feature}")]
    UnsupportedToleranceClass { feature: String, code: String },

    #[error("ISO 286 database not found. Run the ISO import script first.")]
    DatabaseNotFound,

    #[error("No ISO 286 data found for {feature} {code} at {nominal_mm} mm")]
    ToleranceNotFound {
        feature: String,
        code: String,
        nominal_mm: f64,
    },

    #[error("ISO 286 database error: {0}")]
    Database(String),
}

impl From<rusqlite::Error> for Iso286Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err.to_string())
    }
}
