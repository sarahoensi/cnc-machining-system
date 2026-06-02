use rusqlite::{params, Connection, OpenFlags};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::Manager;
use thiserror::Error;

const DB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/iso286.sqlite");
const MISSING_DATABASE_MESSAGE: &str =
    "ISO 286 database not found. Run the ISO import script first.";

#[derive(Debug, Error)]
pub enum Iso286Error {
    #[error("Nominal size must be greater than zero")]
    InvalidNominalSize,

    #[error("Invalid tolerance code '{0}'. Use letters followed by digits, for example H7, JS7, g6, or js6")]
    InvalidToleranceCode(String),

    #[error("Expected feature to be 'hole' or 'shaft', got '{0}'")]
    InvalidFeature(String),

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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ToleranceResult {
    pub code: String,
    pub zone: String,
    pub grade: i32,
    pub upper_um: f64,
    pub lower_um: f64,
    pub min_mm: f64,
    pub max_mm: f64,
    pub source_table: Option<String>,
    pub source_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FitSummary {
    pub min_clearance_mm: f64,
    pub max_clearance_mm: f64,
    #[serde(rename = "type")]
    pub fit_type: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FitResult {
    pub nominal_mm: f64,
    pub hole: ToleranceResult,
    pub shaft: ToleranceResult,
    pub fit: FitSummary,
}

pub fn parse_tolerance_code(code: &str) -> Result<(String, i32), String> {
    let trimmed = code.trim();
    if trimmed.is_empty() {
        return Err("Invalid tolerance code ''. Use letters followed by digits, for example H7, JS7, g6, or js6".to_string());
    }

    let split_idx = trimmed
        .find(|ch: char| ch.is_ascii_digit())
        .ok_or_else(|| invalid_code_message(code))?;
    let (zone, grade_text) = trimmed.split_at(split_idx);

    if zone.is_empty()
        || grade_text.is_empty()
        || !zone.chars().all(|ch| ch.is_ascii_alphabetic())
        || !grade_text.chars().all(|ch| ch.is_ascii_digit())
    {
        return Err(invalid_code_message(code));
    }

    let grade = grade_text
        .parse::<i32>()
        .map_err(|_| invalid_code_message(code))?;

    Ok((zone.to_string(), grade))
}

fn invalid_code_message(code: &str) -> String {
    format!(
        "Invalid tolerance code '{}'. Use letters followed by digits, for example H7, JS7, g6, or js6",
        code
    )
}

pub fn lookup_tolerance(
    db_path: &Path,
    nominal_mm: f64,
    feature: &str,
    code: &str,
) -> Result<ToleranceResult, String> {
    let conn = open_database_read_only(db_path).map_err(|err| err.to_string())?;
    lookup_tolerance_with_connection(&conn, nominal_mm, feature, code)
        .map_err(|err| err.to_string())
}

pub fn lookup_tolerance_with_connection(
    conn: &Connection,
    nominal_mm: f64,
    feature: &str,
    code: &str,
) -> Result<ToleranceResult, Iso286Error> {
    if nominal_mm <= 0.0 {
        return Err(Iso286Error::InvalidNominalSize);
    }
    if feature != "hole" && feature != "shaft" {
        return Err(Iso286Error::InvalidFeature(feature.to_string()));
    }

    let (zone, grade) =
        parse_tolerance_code(code).map_err(|_| Iso286Error::InvalidToleranceCode(code.into()))?;

    let row = conn
        .query_row(
            "SELECT upper_um, lower_um, source_table, source_file
             FROM tolerance_zones
             WHERE feature = ?1
               AND zone = ?2
               AND grade = ?3
               AND ?4 > size_min
               AND ?5 <= size_max
             LIMIT 1",
            params![feature, zone, grade, nominal_mm, nominal_mm],
            |row| {
                Ok((
                    row.get::<_, f64>(0)?,
                    row.get::<_, f64>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            },
        )
        .map_err(|err| match err {
            rusqlite::Error::QueryReturnedNoRows => Iso286Error::ToleranceNotFound {
                feature: feature.to_string(),
                code: code.trim().to_string(),
                nominal_mm,
            },
            other => other.into(),
        })?;

    let (upper_um, lower_um, source_table, source_file) = row;
    Ok(ToleranceResult {
        code: code.trim().to_string(),
        zone,
        grade,
        upper_um,
        lower_um,
        min_mm: nominal_mm + lower_um / 1000.0,
        max_mm: nominal_mm + upper_um / 1000.0,
        source_table,
        source_file,
    })
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

pub fn open_database_read_only(path: &Path) -> Result<Connection, Iso286Error> {
    if !path.exists() {
        return Err(Iso286Error::DatabaseNotFound);
    }

    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(Into::into)
}

fn bundled_or_development_db_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .resource_dir()
        .ok()
        .map(|dir| dir.join("data").join("iso286.sqlite"))
        .filter(|path| path.exists())
        .unwrap_or_else(|| Path::new(DB_PATH).to_path_buf())
}

#[tauri::command]
pub fn calculate_iso286_fit(
    app: tauri::AppHandle,
    nominal_mm: f64,
    hole: String,
    shaft: String,
) -> Result<FitResult, String> {
    let db_path = bundled_or_development_db_path(&app);
    if !db_path.exists() {
        return Err(MISSING_DATABASE_MESSAGE.to_string());
    }

    calculate_fit(&db_path, nominal_mm, &hole, &shaft)
}
