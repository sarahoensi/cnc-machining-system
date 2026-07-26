use std::path::Path;

use rusqlite::{params, Connection, OpenFlags};

use crate::application::thread::dto::{
    SolveThreadOutput, ThreadOptionsOutput, ThreadPitchOptionOutput, ThreadSizeOptionOutput,
    ThreadTypeOptionOutput,
};
use crate::domain::machining::thread::{ThreadSpec, ThreadType};

pub fn open_thread_database_read_only(path: &Path) -> Result<Connection, String> {
    if !path.exists() {
        return Err("Thread database not found. Run the thread import script first.".to_string());
    }

    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|err| err.to_string())
}

pub fn list_thread_options(db_path: &Path) -> Result<ThreadOptionsOutput, String> {
    let conn = open_thread_database_read_only(db_path)?;

    Ok(ThreadOptionsOutput {
        types: vec![
            ThreadTypeOptionOutput {
                value: "metric".to_string(),
                label: "Metric".to_string(),
            },
            ThreadTypeOptionOutput {
                value: "unc".to_string(),
                label: "UNC".to_string(),
            },
            ThreadTypeOptionOutput {
                value: "unf".to_string(),
                label: "UNF".to_string(),
            },
            ThreadTypeOptionOutput {
                value: "bsp".to_string(),
                label: "G/BSP".to_string(),
            },
        ],
        metric: list_sizes_for_type(&conn, "metric")?,
        unc: list_sizes_for_type(&conn, "unc")?,
        unf: list_sizes_for_type(&conn, "unf")?,
        bsp: list_sizes_for_type(&conn, "bsp")?,
    })
}

pub fn solve_thread(
    db_path: &Path,
    thread_type: &str,
    size: &str,
    pitch: &str,
) -> Result<SolveThreadOutput, String> {
    let conn = open_thread_database_read_only(db_path)?;
    let spec = lookup_thread_spec_with_connection(&conn, thread_type, size, pitch)?;
    let result = crate::domain::machining::thread::ThreadSolver::solve(&spec);

    Ok(SolveThreadOutput {
        drill_diameter_mm: result.drill_diameter_mm,
        thread_depth_mm: result.thread_depth_mm,
    })
}

fn list_sizes_for_type(
    conn: &Connection,
    thread_type: &str,
) -> Result<Vec<ThreadSizeOptionOutput>, String> {
    let mut statement = conn
        .prepare(
            "SELECT designation, MIN(major_diameter_mm)
             FROM thread_specs
             WHERE thread_type = ?1
             GROUP BY designation
             ORDER BY MIN(major_diameter_mm), designation",
        )
        .map_err(|err| err.to_string())?;

    let sizes = statement
        .query_map(params![thread_type], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })
        .map_err(|err| err.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;

    sizes
        .into_iter()
        .map(|(designation, major_diameter_mm)| {
            Ok(ThreadSizeOptionOutput {
                value: designation.clone(),
                label: designation.clone(),
                major_diameter_mm,
                pitches: list_pitches_for_size(conn, thread_type, &designation)?,
            })
        })
        .collect()
}

fn list_pitches_for_size(
    conn: &Connection,
    thread_type: &str,
    designation: &str,
) -> Result<Vec<ThreadPitchOptionOutput>, String> {
    let mut statement = conn
        .prepare(
            "SELECT pitch_mm, tpi, display_name, series, is_default_pitch
             FROM thread_specs
             WHERE thread_type = ?1
               AND designation = ?2
             ORDER BY is_default_pitch DESC, pitch_mm",
        )
        .map_err(|err| err.to_string())?;

    let pitches = statement
        .query_map(params![thread_type, designation], |row| {
            let pitch_mm = row.get::<_, f64>(0)?;
            let tpi = row.get::<_, Option<f64>>(1)?;
            let display_name = row.get::<_, String>(2)?;
            let series = row.get::<_, String>(3)?;
            let is_default_pitch = row.get::<_, i32>(4)? == 1;

            Ok(ThreadPitchOptionOutput {
                value: pitch_value(thread_type, pitch_mm, tpi),
                label: pitch_label(thread_type, pitch_mm, tpi, &display_name),
                pitch_mm,
                series,
                is_default_pitch,
            })
        })
        .map_err(|err| err.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;

    Ok(pitches)
}

fn lookup_thread_spec_with_connection(
    conn: &Connection,
    thread_type: &str,
    size: &str,
    pitch: &str,
) -> Result<ThreadSpec, String> {
    let parsed_thread_type = thread_type
        .parse::<ThreadType>()
        .map_err(|err| err.to_string())?;

    let mut statement = conn
        .prepare(
            "SELECT designation, pitch_mm, tpi, major_diameter_mm, tap_drill_mm, radial_thread_depth_mm
             FROM thread_specs
             WHERE thread_type = ?1
               AND designation = ?2
             ORDER BY pitch_mm",
        )
        .map_err(|err| err.to_string())?;

    let rows = statement
        .query_map(params![thread_type, size], |row| {
            let pitch_mm = row.get::<_, f64>(1)?;
            let tpi = row.get::<_, Option<f64>>(2)?;
            Ok((
                row.get::<_, String>(0)?,
                pitch_mm,
                tpi,
                row.get::<_, f64>(3)?,
                row.get::<_, f64>(4)?,
                row.get::<_, f64>(5)?,
            ))
        })
        .map_err(|err| err.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;

    if rows.is_empty() {
        return Err(format!(
            "Unsupported thread size '{size}' for {thread_type}"
        ));
    }

    let selected = rows
        .into_iter()
        .find(|(_, pitch_mm, tpi, _, _, _)| pitch_value(thread_type, *pitch_mm, *tpi) == pitch)
        .ok_or_else(|| {
            format!("Unsupported pitch '{pitch}' for {thread_type} thread size {size}")
        })?;

    Ok(ThreadSpec {
        thread_type: parsed_thread_type,
        size: selected.0,
        pitch: pitch.to_string(),
        pitch_mm: selected.1,
        major_diameter_mm: selected.3,
        tap_drill_mm: selected.4,
        radial_thread_depth_mm: selected.5,
    })
}

fn pitch_value(thread_type: &str, pitch_mm: f64, tpi: Option<f64>) -> String {
    if thread_type == "metric" {
        format_float(pitch_mm)
    } else {
        tpi.map(format_float)
            .unwrap_or_else(|| format_float(pitch_mm))
    }
}

fn pitch_label(thread_type: &str, pitch_mm: f64, tpi: Option<f64>, display_name: &str) -> String {
    if thread_type == "metric" {
        format!("{} mm", format_float(pitch_mm))
    } else {
        tpi.map(|value| format!("{} TPI", format_float(value)))
            .unwrap_or_else(|| display_name.to_string())
    }
}

fn format_float(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        let text = format!("{value:.4}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
