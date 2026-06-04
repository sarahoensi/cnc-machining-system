use rusqlite::Connection;
use std::path::Path;

use crate::application::tolerance::{
    dto::{FitResult, FitSummary},
    error::Iso286Error,
    repository::{lookup_tolerance_with_connection, open_database_read_only},
};

pub fn calculate_fit(
    db_path: &Path,
    nominal_mm: f64,
    hole_code: &str,
    shaft_code: &str,
) -> Result<FitResult, String> {
    let conn = open_database_read_only(db_path).map_err(|err| err.to_string())?;
    calculate_fit_with_connection(&conn, nominal_mm, hole_code, shaft_code)
        .map_err(|err| err.to_string())
}

pub fn calculate_fit_with_connection(
    conn: &Connection,
    nominal_mm: f64,
    hole_code: &str,
    shaft_code: &str,
) -> Result<FitResult, Iso286Error> {
    if nominal_mm <= 0.0 {
        return Err(Iso286Error::InvalidNominalSize);
    }

    let hole = lookup_tolerance_with_connection(conn, nominal_mm, "hole", hole_code)?;
    let shaft = lookup_tolerance_with_connection(conn, nominal_mm, "shaft", shaft_code)?;

    let min_clearance_mm = hole.min_mm - shaft.max_mm;
    let max_clearance_mm = hole.max_mm - shaft.min_mm;
    let fit_type = if min_clearance_mm >= 0.0 {
        "clearance"
    } else if max_clearance_mm <= 0.0 {
        "interference"
    } else {
        "transition"
    };

    Ok(FitResult {
        nominal_mm,
        hole,
        shaft,
        fit: FitSummary {
            min_clearance_mm,
            max_clearance_mm,
            fit_type: fit_type.to_string(),
        },
    })
}
