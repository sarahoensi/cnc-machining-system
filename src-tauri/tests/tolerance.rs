use cnc_machining_system_lib::application::{
    calculate_fit_with_connection, list_tolerance_options_with_connection, lookup_tolerance,
    lookup_tolerance_with_connection, parse_tolerance_code, Iso286Error,
};
use rusqlite::Connection;
use std::path::Path;

fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "{left} != {right}");
}

#[test]
fn parses_tolerance_codes() {
    assert_eq!(parse_tolerance_code("H7").unwrap(), ("H".to_string(), 7));
    assert_eq!(parse_tolerance_code("JS7").unwrap(), ("JS".to_string(), 7));
    assert_eq!(parse_tolerance_code("g6").unwrap(), ("g".to_string(), 6));
    assert_eq!(parse_tolerance_code("js6").unwrap(), ("js".to_string(), 6));
    assert_eq!(parse_tolerance_code("r6").unwrap(), ("r".to_string(), 6));
}

#[test]
fn rejects_invalid_tolerance_codes() {
    assert!(parse_tolerance_code("").is_err());
    assert!(parse_tolerance_code("7H").is_err());
    assert!(parse_tolerance_code("H").is_err());
    assert!(parse_tolerance_code("H7x").is_err());
}

#[test]
fn missing_database_has_clear_error() {
    let result = lookup_tolerance(
        Path::new("does-not-exist/iso286.sqlite"),
        42.0,
        "hole",
        "H7",
    );

    assert_eq!(
        result.unwrap_err(),
        "ISO 286 database not found. Run the ISO import script first."
    );
}

#[test]
fn calculates_h7_g6_at_42_mm() {
    let conn = fixture_connection();
    let result = calculate_fit_with_connection(&conn, 42.0, "H7", "g6").unwrap();

    assert_eq!(result.nominal_mm, 42.0);
    assert_eq!(result.hole.zone, "H");
    assert_eq!(result.hole.grade, 7);
    assert_close(result.hole.upper_um, 25.0);
    assert_close(result.hole.lower_um, 0.0);
    assert_close(result.hole.mid_um, 12.5);
    assert_close(result.hole.min_mm, 42.0);
    assert_close(result.hole.max_mm, 42.025);
    assert_close(result.hole.mid_mm, 42.0125);
    assert_eq!(result.hole.source_table.as_deref(), Some("Table 6"));
    assert_eq!(result.hole.source_file.as_deref(), Some("holes_h.csv"));

    assert_eq!(result.shaft.zone, "g");
    assert_eq!(result.shaft.grade, 6);
    assert_close(result.shaft.upper_um, -9.0);
    assert_close(result.shaft.lower_um, -25.0);
    assert_close(result.shaft.mid_um, -17.0);
    assert_close(result.shaft.min_mm, 41.975);
    assert_close(result.shaft.max_mm, 41.991);
    assert_close(result.shaft.mid_mm, 41.983);
    assert_eq!(result.shaft.source_table.as_deref(), Some("Table 21"));
    assert_eq!(result.shaft.source_file.as_deref(), Some("shafts_g.csv"));

    assert_close(result.fit.min_clearance_mm, 0.009);
    assert_close(result.fit.max_clearance_mm, 0.05);
    assert_eq!(result.fit.fit_type, "clearance");
}

#[test]
fn calculates_h7_h6_at_42_mm() {
    let conn = fixture_connection();
    let result = calculate_fit_with_connection(&conn, 42.0, "H7", "h6").unwrap();

    assert_close(result.shaft.upper_um, 0.0);
    assert_close(result.shaft.lower_um, -16.0);
    assert_close(result.fit.min_clearance_mm, 0.0);
    assert_close(result.fit.max_clearance_mm, 0.041);
    assert_eq!(result.fit.fit_type, "clearance");
}

#[test]
fn calculates_d6_h6_at_42_mm() {
    let conn = fixture_connection();
    let result = calculate_fit_with_connection(&conn, 42.0, "D6", "h6").unwrap();

    assert_eq!(result.hole.zone, "D");
    assert_eq!(result.hole.grade, 6);
    assert_close(result.hole.upper_um, 26.0);
    assert_close(result.hole.lower_um, 20.0);
    assert_eq!(result.hole.source_table.as_deref(), Some("Table 3"));
    assert_eq!(result.hole.source_file.as_deref(), Some("table_3.csv"));
    assert_eq!(result.fit.fit_type, "clearance");
}

#[test]
fn looks_up_single_hole_and_shaft_tolerances() {
    let conn = fixture_connection();

    let hole = lookup_tolerance_with_connection(&conn, 42.0, "hole", "JS7").unwrap();
    assert_eq!(hole.zone, "JS");
    assert_eq!(hole.grade, 7);
    assert_close(hole.upper_um, 12.5);
    assert_close(hole.lower_um, -12.5);
    assert_close(hole.mid_um, 0.0);
    assert_close(hole.mid_mm, 42.0);

    let shaft = lookup_tolerance_with_connection(&conn, 42.0, "shaft", "p6").unwrap();
    assert_eq!(shaft.zone, "p");
    assert_eq!(shaft.grade, 6);
    assert_close(shaft.upper_um, 42.0);
    assert_close(shaft.lower_um, 26.0);
    assert_close(shaft.mid_um, 34.0);
    assert_close(shaft.mid_mm, 42.034);
}

#[test]
fn lists_only_supported_tolerance_options() {
    let conn = fixture_connection();
    let options = list_tolerance_options_with_connection(&conn).unwrap();

    let hole_zones: Vec<_> = options
        .holes
        .iter()
        .map(|option| option.zone.as_str())
        .collect();
    let shaft_zones: Vec<_> = options
        .shafts
        .iter()
        .map(|option| option.zone.as_str())
        .collect();

    assert_eq!(hole_zones, vec!["D", "H", "JS", "ZA"]);
    assert_eq!(shaft_zones, vec!["g", "h", "js", "p", "r"]);
    assert_eq!(options.holes[2].grades, vec![7]);
    assert_eq!(options.shafts[4].grades, vec![6]);
}

#[test]
fn rejects_classes_outside_supported_allowlist() {
    let conn = fixture_connection();
    let result = lookup_tolerance_with_connection(&conn, 42.0, "hole", "CD6");

    assert!(matches!(
        result,
        Err(Iso286Error::UnsupportedToleranceClass { .. })
    ));
}

#[test]
fn upper_bound_uses_current_interval() {
    let conn = fixture_connection();
    let result = calculate_fit_with_connection(&conn, 50.0, "H7", "g6").unwrap();

    assert_close(result.hole.max_mm, 50.025);
}

#[test]
fn value_above_fixture_interval_fails() {
    let conn = fixture_connection();
    let result = calculate_fit_with_connection(&conn, 50.001, "H7", "g6");

    assert!(matches!(result, Err(Iso286Error::ToleranceNotFound { .. })));
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No ISO 286 data found for hole H7 at 50.001 mm"));
}

#[test]
fn hh7_is_invalid_when_zone_does_not_exist_in_database() {
    let conn = fixture_connection();
    let result = calculate_fit_with_connection(&conn, 42.0, "HH7", "g6");

    assert!(matches!(
        result,
        Err(Iso286Error::UnsupportedToleranceClass { .. })
    ));
}

fn fixture_connection() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "
        CREATE TABLE tolerance_zones(
          id INTEGER PRIMARY KEY,
          feature TEXT NOT NULL CHECK(feature IN ('hole', 'shaft')),
          zone TEXT NOT NULL,
          grade INTEGER NOT NULL,
          size_min REAL NOT NULL,
          size_max REAL NOT NULL,
          upper_um REAL NOT NULL,
          lower_um REAL NOT NULL,
          source_table TEXT,
          source_file TEXT,
          CHECK(size_min < size_max),
          CHECK(upper_um >= lower_um),
          UNIQUE(feature, zone, grade, size_min, size_max)
        );

        CREATE INDEX idx_tolerance_lookup
        ON tolerance_zones(feature, zone, grade, size_min, size_max);

        CREATE TABLE metadata(
          key TEXT PRIMARY KEY,
          value TEXT NOT NULL
        );

        INSERT INTO tolerance_zones(
          feature, zone, grade, size_min, size_max, upper_um, lower_um, source_table, source_file
        ) VALUES
          ('hole', 'H', 7, 30.0, 50.0, 25.0, 0.0, 'Table 6', 'holes_h.csv'),
          ('hole', 'D', 6, 30.0, 50.0, 26.0, 20.0, 'Table 3', 'table_3.csv'),
          ('hole', 'JS', 7, 30.0, 50.0, 12.5, -12.5, 'Table 7', 'table_7.csv'),
          ('hole', 'ZA', 8, 30.0, 50.0, 160.0, 120.0, 'Table 15', 'table_15.csv'),
          ('hole', 'CD', 6, 30.0, 50.0, 80.0, 64.0, 'Table extra', 'extra.csv'),
          ('shaft', 'g', 6, 30.0, 50.0, -9.0, -25.0, 'Table 21', 'shafts_g.csv'),
          ('shaft', 'h', 6, 30.0, 50.0, 0.0, -16.0, 'Table 22', 'shafts_h.csv'),
          ('shaft', 'js', 6, 30.0, 50.0, 8.0, -8.0, 'Table 23', 'table_23.csv'),
          ('shaft', 'p', 6, 30.0, 50.0, 42.0, 26.0, 'Table 26', 'table_26.csv'),
          ('shaft', 'r', 6, 30.0, 50.0, 50.0, 34.0, 'Table 27', 'table_27.csv');
        ",
    )
    .unwrap();
    conn
}
