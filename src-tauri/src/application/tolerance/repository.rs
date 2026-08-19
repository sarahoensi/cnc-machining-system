use rusqlite::{params, Connection, OpenFlags};
use std::path::Path;

use crate::{
    application::tolerance::{
        dto::{ToleranceOption, ToleranceOptions, ToleranceResult},
        error::Iso286Error,
    },
    domain::machining::tolerance::{
        is_allowed_zone, parse_tolerance_code, REQUIRED_HOLE_ZONES, REQUIRED_SHAFT_ZONES,
    },
};

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
    if !is_allowed_zone(feature, &zone) {
        return Err(Iso286Error::UnsupportedToleranceClass {
            feature: feature.to_string(),
            code: code.trim().to_string(),
        });
    }

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
    let mid_um = (upper_um + lower_um) / 2.0;
    let min_mm = nominal_mm + lower_um / 1000.0;
    let max_mm = nominal_mm + upper_um / 1000.0;
    let mid_mm = (max_mm + min_mm) / 2.0;

    Ok(ToleranceResult {
        code: code.trim().to_string(),
        zone,
        grade,
        upper_um,
        lower_um,
        mid_um,
        min_mm,
        max_mm,
        mid_mm,
        source_table,
        source_file,
    })
}

pub fn list_tolerance_options(db_path: &Path) -> Result<ToleranceOptions, String> {
    let conn = open_database_read_only(db_path).map_err(|err| err.to_string())?;
    list_tolerance_options_with_connection(&conn).map_err(|err| err.to_string())
}

pub fn list_tolerance_options_with_connection(
    conn: &Connection,
) -> Result<ToleranceOptions, Iso286Error> {
    Ok(ToleranceOptions {
        holes: list_options_for_feature(conn, "hole", REQUIRED_HOLE_ZONES)?,
        shafts: list_options_for_feature(conn, "shaft", REQUIRED_SHAFT_ZONES)?,
    })
}

fn list_options_for_feature(
    conn: &Connection,
    feature: &str,
    zones: &[&str],
) -> Result<Vec<ToleranceOption>, Iso286Error> {
    let mut options = Vec::new();
    for zone in zones {
        let mut statement = conn.prepare(
            "SELECT DISTINCT grade
             FROM tolerance_zones
             WHERE feature = ?1
               AND zone = ?2
             ORDER BY grade",
        )?;
        let grades = statement
            .query_map(params![feature, zone], |row| row.get::<_, i32>(0))?
            .collect::<Result<Vec<_>, _>>()?;

        if !grades.is_empty() {
            options.push(ToleranceOption {
                feature: feature.to_string(),
                zone: (*zone).to_string(),
                grades,
            });
        }
    }
    Ok(options)
}
