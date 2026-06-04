use std::path::{Path, PathBuf};

use tauri::{command, Manager};

use crate::application::{calculate_fit, list_tolerance_options, lookup_tolerance};

use super::{FitResponse, ToleranceOptionsResponse, ToleranceResponse};

const DB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/iso286.sqlite");
const MISSING_DATABASE_MESSAGE: &str =
    "ISO 286 database not found. Run the ISO import script first.";

fn bundled_or_development_db_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .resource_dir()
        .ok()
        .map(|dir| dir.join("data").join("iso286.sqlite"))
        .filter(|path| path.exists())
        .unwrap_or_else(|| Path::new(DB_PATH).to_path_buf())
}

#[command]
pub fn calculate_iso286_fit(
    app: tauri::AppHandle,
    nominal_mm: f64,
    hole: String,
    shaft: String,
) -> Result<FitResponse, String> {
    let db_path = bundled_or_development_db_path(&app);
    if !db_path.exists() {
        return Err(MISSING_DATABASE_MESSAGE.to_string());
    }

    calculate_fit(&db_path, nominal_mm, &hole, &shaft).map(Into::into)
}

#[command]
pub fn lookup_iso286_tolerance(
    app: tauri::AppHandle,
    feature: String,
    nominal_mm: f64,
    code: String,
) -> Result<ToleranceResponse, String> {
    let db_path = bundled_or_development_db_path(&app);
    if !db_path.exists() {
        return Err(MISSING_DATABASE_MESSAGE.to_string());
    }

    lookup_tolerance(&db_path, nominal_mm, &feature, &code).map(Into::into)
}

#[command]
pub fn list_iso286_tolerance_options(
    app: tauri::AppHandle,
) -> Result<ToleranceOptionsResponse, String> {
    let db_path = bundled_or_development_db_path(&app);
    if !db_path.exists() {
        return Err(MISSING_DATABASE_MESSAGE.to_string());
    }

    list_tolerance_options(&db_path).map(Into::into)
}
