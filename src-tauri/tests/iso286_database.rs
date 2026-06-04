use cnc_machining_system_lib::application::{
    calculate_fit_with_connection, list_tolerance_options, lookup_tolerance,
    lookup_tolerance_with_connection, parse_tolerance_code, Iso286Error, ToleranceOptions,
};
use rusqlite::{params, Connection, OpenFlags};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const REQUIRED_HOLE_ZONES: &[&str] = &[
    "D", "E", "F", "G", "H", "J", "JS", "K", "M", "N", "P", "R", "S", "T", "U", "V", "X", "Y", "Z",
    "ZA", "ZB", "ZC",
];
const REQUIRED_SHAFT_ZONES: &[&str] = &["f", "g", "h", "js", "k", "m", "n", "p", "r"];
const STANDARD_INTERVALS: &[(f64, f64)] = &[
    (0.0, 3.0),
    (3.0, 6.0),
    (6.0, 10.0),
    (10.0, 18.0),
    (18.0, 30.0),
    (30.0, 50.0),
    (50.0, 80.0),
    (80.0, 120.0),
    (120.0, 180.0),
    (180.0, 250.0),
    (250.0, 315.0),
    (315.0, 400.0),
    (400.0, 500.0),
    (500.0, 630.0),
    (630.0, 800.0),
    (800.0, 1000.0),
    (1000.0, 1250.0),
    (1250.0, 1600.0),
    (1600.0, 2000.0),
    (2000.0, 2500.0),
    (2500.0, 3150.0),
];

#[derive(Clone, Copy)]
struct GoldenTolerance {
    feature: &'static str,
    zone: &'static str,
    grade: i32,
    nominal_mm: f64,
    size_min: f64,
    size_max: f64,
    upper_um: f64,
    lower_um: f64,
    source_table: &'static str,
    source_file: &'static str,
}

#[derive(Clone, Copy)]
struct FitCase {
    nominal_mm: f64,
    hole_code: &'static str,
    shaft_code: &'static str,
    hole_upper_um: f64,
    hole_lower_um: f64,
    shaft_upper_um: f64,
    shaft_lower_um: f64,
    min_clearance_mm: f64,
    max_clearance_mm: f64,
    fit_type: &'static str,
}

fn database_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("iso286.sqlite")
}

fn open_database() -> Connection {
    let path = database_path();
    Connection::open_with_flags(
        &path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()))
}

fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "{left} != {right}");
}

fn assert_optional_close(left: Option<f64>, right: Option<f64>) {
    match (left, right) {
        (Some(left), Some(right)) => assert_close(left, right),
        (None, None) => {}
        _ => panic!("{left:?} != {right:?}"),
    }
}

#[test]
fn database_file_exists_and_has_expected_metadata() {
    let path = database_path();
    assert!(
        path.exists(),
        "missing ISO 286 database at {}",
        path.display()
    );

    let conn = open_database();
    let metadata: HashMap<String, String> = conn
        .prepare("SELECT key, value FROM metadata")
        .unwrap()
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();

    assert_eq!(
        metadata.get("standard").map(String::as_str),
        Some("ISO 286-2:2010")
    );
    assert_eq!(
        metadata.get("import_model").map(String::as_str),
        Some("full_table_lookup")
    );
    assert_eq!(
        metadata.get("generated_from").map(String::as_str),
        Some("local_csv_files")
    );
    assert_eq!(
        metadata.get("imported_tables").map(String::as_str),
        Some("1,3,4,5,6,7,8,9,10,11,12,13,14,15,16,20,21,22,23,24,25,26,27")
    );
    assert!(
        metadata.contains_key("generated_at"),
        "database metadata must include generated_at"
    );
}

#[test]
fn imported_iso_tables_have_expected_row_counts() {
    let conn = open_database();

    let tolerance_rows: i64 = conn
        .query_row("SELECT COUNT(*) FROM tolerance_zones", [], |row| row.get(0))
        .unwrap();
    let it_grade_rows: i64 = conn
        .query_row("SELECT COUNT(*) FROM iso_it_grades", [], |row| row.get(0))
        .unwrap();
    let tolerance_classes: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT feature || ':' || zone || ':' || grade) FROM tolerance_zones",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(tolerance_rows, 4889);
    assert_eq!(it_grade_rows, 21);
    assert_eq!(tolerance_classes, 291);

    let expected = [
        ("Table 3", 301),
        ("Table 4", 139),
        ("Table 5", 124),
        ("Table 6", 378),
        ("Table 7", 376),
        ("Table 8", 129),
        ("Table 9", 235),
        ("Table 10", 115),
        ("Table 11", 160),
        ("Table 12", 176),
        ("Table 13", 284),
        ("Table 14", 254),
        ("Table 15", 268),
        ("Table 16", 250),
        ("Table 20", 146),
        ("Table 21", 107),
        ("Table 22", 376),
        ("Table 23", 376),
        ("Table 24", 240),
        ("Table 25", 172),
        ("Table 26", 107),
        ("Table 27", 176),
    ];

    for (table, expected_rows) in expected {
        let actual_rows: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM tolerance_zones WHERE source_table = ?1",
                [table],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(actual_rows, expected_rows, "{table} row count changed");
    }
}

#[test]
fn all_standard_size_intervals_exist_and_tolerance_rows_use_only_those_intervals() {
    let conn = open_database();
    let it_ranges = conn
        .prepare("SELECT size_min, size_max FROM iso_it_grades ORDER BY size_min, size_max")
        .unwrap()
        .query_map([], |row| Ok((row.get::<_, f64>(0)?, row.get::<_, f64>(1)?)))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(it_ranges, STANDARD_INTERVALS);

    let ranges_outside_standard_coverage: i64 = conn
        .query_row(
            "SELECT COUNT(*)
             FROM tolerance_zones tz
             LEFT JOIN iso_it_grades it
               ON tz.size_min >= it.size_min AND tz.size_max <= it.size_max
             WHERE it.id IS NULL",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(ranges_outside_standard_coverage, 0);
}

#[test]
fn tolerance_intervals_do_not_overlap_within_a_class() {
    let conn = open_database();
    let classes = conn
        .prepare(
            "SELECT DISTINCT feature, zone, grade
             FROM tolerance_zones
             ORDER BY feature, zone, grade",
        )
        .unwrap()
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
            ))
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for (feature, zone, grade) in classes {
        let ranges = conn
            .prepare(
                "SELECT size_min, size_max
                 FROM tolerance_zones
                 WHERE feature = ?1 AND zone = ?2 AND grade = ?3
                 ORDER BY size_min, size_max",
            )
            .unwrap()
            .query_map(params![feature, zone, grade], |row| {
                Ok((row.get::<_, f64>(0)?, row.get::<_, f64>(1)?))
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        for pair in ranges.windows(2) {
            assert!(
                pair[0].1 <= pair[1].0,
                "{feature} {zone}{grade} has overlapping intervals {:?} and {:?}",
                pair[0],
                pair[1]
            );
        }
    }
}

#[test]
fn size_interval_boundaries_are_above_min_and_up_to_including_max() {
    let db_path = database_path();

    let cases = [
        (3.0, 0.0, 3.0, 10.0),
        (3.001, 3.0, 6.0, 12.0),
        (6.0, 3.0, 6.0, 12.0),
        (6.001, 6.0, 10.0, 15.0),
        (500.0, 400.0, 500.0, 63.0),
        (500.001, 500.0, 630.0, 70.0),
    ];

    for (nominal_mm, expected_min, expected_max, expected_es) in cases {
        let row = lookup_row("hole", "H", 7, nominal_mm);
        assert_close(row.0, expected_min);
        assert_close(row.1, expected_max);

        let result = lookup_tolerance(&db_path, nominal_mm, "hole", "H7").unwrap();
        assert_close(result.upper_um, expected_es);
    }
}

#[test]
fn null_values_exist_only_where_iso_it_table_has_no_grade() {
    let conn = open_database();

    let tolerance_nulls: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tolerance_zones
             WHERE upper_um IS NULL OR lower_um IS NULL",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(tolerance_nulls, 0);

    for column in ["IT01", "IT0", "IT1"] {
        let nulls_below_500: i64 = conn
            .query_row(
                &format!(
                    "SELECT COUNT(*) FROM iso_it_grades WHERE {column} IS NULL AND size_min < 500"
                ),
                [],
                |row| row.get(0),
            )
            .unwrap();
        let defined_at_or_above_500: i64 = conn
            .query_row(
                &format!("SELECT COUNT(*) FROM iso_it_grades WHERE {column} IS NOT NULL AND size_min >= 500"),
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(nulls_below_500, 0);
        assert_eq!(defined_at_or_above_500, 0);
    }

    for column in [
        "IT2", "IT3", "IT4", "IT5", "IT6", "IT7", "IT8", "IT9", "IT10", "IT11", "IT12", "IT13",
        "IT14", "IT15", "IT16", "IT17", "IT18",
    ] {
        let nulls: i64 = conn
            .query_row(
                &format!("SELECT COUNT(*) FROM iso_it_grades WHERE {column} IS NULL"),
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(nulls, 0, "{column} unexpectedly contains NULL");
    }
}

#[test]
fn database_contains_all_supported_tolerance_zones() {
    let conn = open_database();

    for zone in REQUIRED_HOLE_ZONES {
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM tolerance_zones WHERE feature = 'hole' AND zone = ?1",
                [zone],
                |row| row.get(0),
            )
            .unwrap();
        assert!(count > 0, "missing supported hole zone {zone}");
    }

    for zone in REQUIRED_SHAFT_ZONES {
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM tolerance_zones WHERE feature = 'shaft' AND zone = ?1",
                [zone],
                |row| row.get(0),
            )
            .unwrap();
        assert!(count > 0, "missing supported shaft zone {zone}");
    }
}

#[test]
fn supported_options_are_available_through_application_api() {
    let options = list_tolerance_options(&database_path()).unwrap();

    assert_contains_zones("hole", &options, REQUIRED_HOLE_ZONES);
    assert_contains_zones("shaft", &options, REQUIRED_SHAFT_ZONES);
}

#[test]
fn golden_tolerance_rows_match_iso286_database_values() {
    for case in golden_tolerance_cases() {
        let row = lookup_row(case.feature, case.zone, case.grade, case.nominal_mm);

        assert_close(row.0, case.size_min);
        assert_close(row.1, case.size_max);
        assert_close(row.2, case.upper_um);
        assert_close(row.3, case.lower_um);
        assert_eq!(row.4, case.source_table);
        assert_eq!(row.5, case.source_file);
    }
}

#[test]
fn golden_it_grade_rows_match_iso286_table_1_values() {
    let conn = open_database();
    let cases = [
        (
            0.0,
            3.0,
            Some(0.3),
            Some(0.5),
            Some(0.8),
            Some(6.0),
            Some(10.0),
            Some(1.4),
        ),
        (
            18.0,
            30.0,
            Some(0.6),
            Some(1.0),
            Some(1.5),
            Some(13.0),
            Some(21.0),
            Some(3.3),
        ),
        (
            30.0,
            50.0,
            Some(0.6),
            Some(1.0),
            Some(1.5),
            Some(16.0),
            Some(25.0),
            Some(3.9),
        ),
        (
            500.0,
            630.0,
            None,
            None,
            None,
            Some(32.0),
            Some(44.0),
            Some(7.0),
        ),
        (
            2500.0,
            3150.0,
            None,
            None,
            None,
            Some(96.0),
            Some(135.0),
            Some(21.0),
        ),
    ];

    for (size_min, size_max, it01, it0, it1, it6, it7, it18) in cases {
        let row = conn
            .query_row(
                "SELECT IT01, IT0, IT1, IT6, IT7, IT18
                 FROM iso_it_grades
                 WHERE size_min = ?1 AND size_max = ?2",
                params![size_min, size_max],
                |row| {
                    Ok((
                        row.get::<_, Option<f64>>(0)?,
                        row.get::<_, Option<f64>>(1)?,
                        row.get::<_, Option<f64>>(2)?,
                        row.get::<_, Option<f64>>(3)?,
                        row.get::<_, Option<f64>>(4)?,
                        row.get::<_, Option<f64>>(5)?,
                    ))
                },
            )
            .unwrap();

        assert_optional_close(row.0, it01);
        assert_optional_close(row.1, it0);
        assert_optional_close(row.2, it1);
        assert_optional_close(row.3, it6);
        assert_optional_close(row.4, it7);
        assert_optional_close(row.5, it18);
    }
}

#[test]
fn h_hole_classes_have_zero_lower_deviation() {
    let conn = open_database();
    let bad_rows: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tolerance_zones
             WHERE feature = 'hole' AND zone = 'H' AND lower_um != 0",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(bad_rows, 0);
}

#[test]
fn h_shaft_classes_have_zero_upper_deviation() {
    let conn = open_database();
    let bad_rows: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tolerance_zones
             WHERE feature = 'shaft' AND zone = 'h' AND upper_um != 0",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(bad_rows, 0);
}

#[test]
fn js_classes_are_symmetric_around_zero() {
    let conn = open_database();
    let bad_rows: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tolerance_zones
             WHERE zone IN ('JS', 'js') AND ABS(upper_um + lower_um) > 0.000000001",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(bad_rows, 0);
}

#[test]
fn every_defined_tolerance_has_positive_width() {
    let conn = open_database();
    let rows = conn
        .prepare(
            "SELECT feature, zone, grade, size_min, size_max, upper_um, lower_um
             FROM tolerance_zones",
        )
        .unwrap()
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, f64>(4)?,
                row.get::<_, f64>(5)?,
                row.get::<_, f64>(6)?,
            ))
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for (feature, zone, grade, size_min, size_max, upper_um, lower_um) in rows {
        assert!(
            upper_um > lower_um,
            "{feature} {zone}{grade} {size_min}-{size_max} has non-positive width"
        );
    }
}

#[test]
fn application_lookup_uses_packed_database_values() {
    let db_path = database_path();

    let hole = lookup_tolerance(&db_path, 42.0, "hole", "H7").unwrap();
    assert_eq!(hole.zone, "H");
    assert_eq!(hole.grade, 7);
    assert_close(hole.upper_um, 25.0);
    assert_close(hole.lower_um, 0.0);
    assert_close(hole.min_mm, 42.0);
    assert_close(hole.max_mm, 42.025);

    let shaft = lookup_tolerance(&db_path, 42.0, "shaft", "g6").unwrap();
    assert_eq!(shaft.zone, "g");
    assert_eq!(shaft.grade, 6);
    assert_close(shaft.upper_um, -9.0);
    assert_close(shaft.lower_um, -25.0);
    assert_close(shaft.min_mm, 41.975);
    assert_close(shaft.max_mm, 41.991);
}

#[test]
fn fit_calculations_use_nominal_plus_deviations_and_clearance_formulas() {
    let conn = open_database();

    for case in fit_cases() {
        let result =
            calculate_fit_with_connection(&conn, case.nominal_mm, case.hole_code, case.shaft_code)
                .unwrap();

        assert_close(result.hole.upper_um, case.hole_upper_um);
        assert_close(result.hole.lower_um, case.hole_lower_um);
        assert_close(result.shaft.upper_um, case.shaft_upper_um);
        assert_close(result.shaft.lower_um, case.shaft_lower_um);

        assert_close(
            result.hole.min_mm,
            case.nominal_mm + case.hole_lower_um / 1000.0,
        );
        assert_close(
            result.hole.max_mm,
            case.nominal_mm + case.hole_upper_um / 1000.0,
        );
        assert_close(
            result.shaft.min_mm,
            case.nominal_mm + case.shaft_lower_um / 1000.0,
        );
        assert_close(
            result.shaft.max_mm,
            case.nominal_mm + case.shaft_upper_um / 1000.0,
        );
        assert_close(result.fit.min_clearance_mm, case.min_clearance_mm);
        assert_close(result.fit.max_clearance_mm, case.max_clearance_mm);
        assert_eq!(result.fit.fit_type, case.fit_type);
    }
}

#[test]
fn integrated_nominal_and_fit_designation_flow_calculates_expected_output() {
    let conn = open_database();
    let (nominal_mm, hole_code, shaft_code) = parse_fit_designation_fixture("50 H7/g6").unwrap();
    let result = calculate_fit_with_connection(&conn, nominal_mm, &hole_code, &shaft_code).unwrap();

    assert_close(result.nominal_mm, 50.0);
    assert_eq!(result.hole.code, "H7");
    assert_eq!(result.shaft.code, "g6");
    assert_close(result.hole.min_mm, 50.0);
    assert_close(result.hole.max_mm, 50.025);
    assert_close(result.shaft.min_mm, 49.975);
    assert_close(result.shaft.max_mm, 49.991);
    assert_close(result.fit.min_clearance_mm, 0.009);
    assert_close(result.fit.max_clearance_mm, 0.05);
    assert_eq!(result.fit.fit_type, "clearance");
}

#[test]
fn rejects_invalid_tolerance_input_and_out_of_range_diameters() {
    let conn = open_database();

    assert!(matches!(
        lookup_tolerance_with_connection(&conn, 50.0, "hole", "X99"),
        Err(Iso286Error::ToleranceNotFound { .. })
    ));
    assert!(matches!(
        lookup_tolerance_with_connection(&conn, 50.0, "hole", "CD6"),
        Err(Iso286Error::UnsupportedToleranceClass { .. })
    ));
    assert!(matches!(
        lookup_tolerance_with_connection(&conn, 3150.001, "hole", "H7"),
        Err(Iso286Error::ToleranceNotFound { .. })
    ));
    assert!(matches!(
        lookup_tolerance_with_connection(&conn, 0.0, "hole", "H7"),
        Err(Iso286Error::InvalidNominalSize)
    ));
    assert!(matches!(
        lookup_tolerance_with_connection(&conn, -1.0, "hole", "H7"),
        Err(Iso286Error::InvalidNominalSize)
    ));
    assert!(parse_tolerance_code("H7/g").is_err());
    assert!(parse_fit_designation_fixture("H7/g").is_err());
    assert!(parse_fit_designation_fixture("50 X99").is_err());
}

#[test]
fn missing_database_value_reports_tolerance_not_found() {
    let conn = sparse_fixture_connection();
    let result = calculate_fit_with_connection(&conn, 42.0, "H7", "g6");

    assert!(matches!(
        result,
        Err(Iso286Error::ToleranceNotFound {
            feature,
            code,
            nominal_mm
        }) if feature == "shaft" && code == "g6" && nominal_mm == 42.0
    ));
}

fn lookup_row(
    feature: &str,
    zone: &str,
    grade: i32,
    nominal_mm: f64,
) -> (f64, f64, f64, f64, String, String) {
    let conn = open_database();
    conn.query_row(
        "SELECT size_min, size_max, upper_um, lower_um, source_table, source_file
         FROM tolerance_zones
         WHERE feature = ?1
           AND zone = ?2
           AND grade = ?3
           AND ?4 > size_min
           AND ?5 <= size_max",
        params![feature, zone, grade, nominal_mm, nominal_mm],
        |row| {
            Ok((
                row.get::<_, f64>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        },
    )
    .unwrap_or_else(|err| {
        panic!("missing row for {feature} {zone}{grade} at {nominal_mm} mm: {err}")
    })
}

fn assert_contains_zones(feature: &str, options: &ToleranceOptions, expected_zones: &[&str]) {
    let actual_zones: Vec<&str> = match feature {
        "hole" => options
            .holes
            .iter()
            .map(|option| option.zone.as_str())
            .collect(),
        "shaft" => options
            .shafts
            .iter()
            .map(|option| option.zone.as_str())
            .collect(),
        _ => unreachable!("invalid feature"),
    };

    for zone in expected_zones {
        assert!(
            actual_zones.contains(zone),
            "application options are missing {feature} zone {zone}"
        );
    }
}

fn sparse_fixture_connection() -> Connection {
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

        INSERT INTO tolerance_zones(
          feature, zone, grade, size_min, size_max, upper_um, lower_um, source_table, source_file
        ) VALUES
          ('hole', 'H', 7, 30.0, 50.0, 25.0, 0.0, 'Table 6', 'table_6.csv');
        ",
    )
    .unwrap();
    conn
}

fn parse_fit_designation_fixture(input: &str) -> Result<(f64, String, String), String> {
    let mut parts = input.split_whitespace();
    let nominal_text = parts.next().ok_or_else(|| "missing nominal".to_string())?;
    let fit_text = parts.next().ok_or_else(|| "missing fit".to_string())?;
    if parts.next().is_some() {
        return Err("too many fields".to_string());
    }

    let nominal_mm = nominal_text
        .replace(',', ".")
        .parse::<f64>()
        .map_err(|_| "invalid nominal".to_string())?;
    if nominal_mm <= 0.0 {
        return Err("nominal must be greater than zero".to_string());
    }

    let (hole_code, shaft_code) = fit_text
        .split_once('/')
        .ok_or_else(|| "missing slash".to_string())?;
    let (hole_zone, _) = parse_tolerance_code(hole_code)?;
    let (shaft_zone, _) = parse_tolerance_code(shaft_code)?;
    if !hole_zone.chars().all(|ch| ch.is_ascii_uppercase())
        || !shaft_zone.chars().all(|ch| ch.is_ascii_lowercase())
    {
        return Err("invalid hole/shaft tolerance zones".to_string());
    }

    Ok((nominal_mm, hole_code.to_string(), shaft_code.to_string()))
}

fn fit_cases() -> [FitCase; 6] {
    [
        FitCase {
            nominal_mm: 50.0,
            hole_code: "H7",
            shaft_code: "g6",
            hole_upper_um: 25.0,
            hole_lower_um: 0.0,
            shaft_upper_um: -9.0,
            shaft_lower_um: -25.0,
            min_clearance_mm: 0.009,
            max_clearance_mm: 0.05,
            fit_type: "clearance",
        },
        FitCase {
            nominal_mm: 50.0,
            hole_code: "H7",
            shaft_code: "h6",
            hole_upper_um: 25.0,
            hole_lower_um: 0.0,
            shaft_upper_um: 0.0,
            shaft_lower_um: -16.0,
            min_clearance_mm: 0.0,
            max_clearance_mm: 0.041,
            fit_type: "clearance",
        },
        FitCase {
            nominal_mm: 50.0,
            hole_code: "H7",
            shaft_code: "p6",
            hole_upper_um: 25.0,
            hole_lower_um: 0.0,
            shaft_upper_um: 42.0,
            shaft_lower_um: 26.0,
            min_clearance_mm: -0.042,
            max_clearance_mm: -0.001,
            fit_type: "interference",
        },
        FitCase {
            nominal_mm: 50.0,
            hole_code: "H7",
            shaft_code: "r6",
            hole_upper_um: 25.0,
            hole_lower_um: 0.0,
            shaft_upper_um: 50.0,
            shaft_lower_um: 34.0,
            min_clearance_mm: -0.05,
            max_clearance_mm: -0.009,
            fit_type: "interference",
        },
        FitCase {
            nominal_mm: 50.0,
            hole_code: "H8",
            shaft_code: "f7",
            hole_upper_um: 39.0,
            hole_lower_um: 0.0,
            shaft_upper_um: -25.0,
            shaft_lower_um: -50.0,
            min_clearance_mm: 0.025,
            max_clearance_mm: 0.089,
            fit_type: "clearance",
        },
        FitCase {
            nominal_mm: 50.0,
            hole_code: "H8",
            shaft_code: "js7",
            hole_upper_um: 39.0,
            hole_lower_um: 0.0,
            shaft_upper_um: 12.5,
            shaft_lower_um: -12.5,
            min_clearance_mm: -0.0125,
            max_clearance_mm: 0.0515,
            fit_type: "transition",
        },
    ]
}

fn golden_tolerance_cases() -> [GoldenTolerance; 56] {
    [
        GoldenTolerance {
            feature: "hole",
            zone: "H",
            grade: 7,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 10.0,
            lower_um: 0.0,
            source_table: "Table 6",
            source_file: "table_6.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "H",
            grade: 7,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 21.0,
            lower_um: 0.0,
            source_table: "Table 6",
            source_file: "table_6.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "H",
            grade: 7,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 25.0,
            lower_um: 0.0,
            source_table: "Table 6",
            source_file: "table_6.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "H",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 44.0,
            lower_um: 0.0,
            source_table: "Table 6",
            source_file: "table_6.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "H",
            grade: 7,
            nominal_mm: 3000.0,
            size_min: 2500.0,
            size_max: 3150.0,
            upper_um: 210.0,
            lower_um: 0.0,
            source_table: "Table 6",
            source_file: "table_6.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "JS",
            grade: 5,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 2.0,
            lower_um: -2.0,
            source_table: "Table 7",
            source_file: "table_7.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "JS",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 6.5,
            lower_um: -6.5,
            source_table: "Table 7",
            source_file: "table_7.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "JS",
            grade: 7,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 12.5,
            lower_um: -12.5,
            source_table: "Table 7",
            source_file: "table_7.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "JS",
            grade: 5,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 16.0,
            lower_um: -16.0,
            source_table: "Table 7",
            source_file: "table_7.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "JS",
            grade: 3,
            nominal_mm: 3000.0,
            size_min: 2500.0,
            size_max: 3150.0,
            upper_um: 25.0,
            lower_um: -25.0,
            source_table: "Table 7",
            source_file: "table_7.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "J",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 2.0,
            lower_um: -4.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "J",
            grade: 7,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 12.0,
            lower_um: -9.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "J",
            grade: 8,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 24.0,
            lower_um: -15.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "K",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 0.0,
            lower_um: -6.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "K",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 2.0,
            lower_um: -11.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "K",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 3.0,
            lower_um: -13.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "K",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 0.0,
            lower_um: -44.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "K",
            grade: 6,
            nominal_mm: 3000.0,
            size_min: 2500.0,
            size_max: 3150.0,
            upper_um: 0.0,
            lower_um: -135.0,
            source_table: "Table 8",
            source_file: "table_8.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "M",
            grade: 3,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: -2.0,
            lower_um: -4.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "M",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: -4.0,
            lower_um: -17.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "M",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: -4.0,
            lower_um: -20.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "M",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: -26.0,
            lower_um: -70.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "N",
            grade: 5,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: -4.0,
            lower_um: -8.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "N",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: -11.0,
            lower_um: -24.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "N",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: -12.0,
            lower_um: -28.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "N",
            grade: 6,
            nominal_mm: 3000.0,
            size_min: 2500.0,
            size_max: 3150.0,
            upper_um: -135.0,
            lower_um: -270.0,
            source_table: "Table 9",
            source_file: "table_9.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "P",
            grade: 3,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: -6.0,
            lower_um: -8.0,
            source_table: "Table 10",
            source_file: "table_10.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "P",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: -18.0,
            lower_um: -31.0,
            source_table: "Table 10",
            source_file: "table_10.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "P",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: -21.0,
            lower_um: -37.0,
            source_table: "Table 10",
            source_file: "table_10.csv",
        },
        GoldenTolerance {
            feature: "hole",
            zone: "P",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: -78.0,
            lower_um: -122.0,
            source_table: "Table 10",
            source_file: "table_10.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "h",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 0.0,
            lower_um: -6.0,
            source_table: "Table 22",
            source_file: "table_22.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "h",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 0.0,
            lower_um: -13.0,
            source_table: "Table 22",
            source_file: "table_22.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "h",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 0.0,
            lower_um: -16.0,
            source_table: "Table 22",
            source_file: "table_22.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "h",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 0.0,
            lower_um: -44.0,
            source_table: "Table 22",
            source_file: "table_22.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "h",
            grade: 3,
            nominal_mm: 3000.0,
            size_min: 2500.0,
            size_max: 3150.0,
            upper_um: 0.0,
            lower_um: -50.0,
            source_table: "Table 22",
            source_file: "table_22.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "js",
            grade: 5,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 2.0,
            lower_um: -2.0,
            source_table: "Table 23",
            source_file: "table_23.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "js",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 6.5,
            lower_um: -6.5,
            source_table: "Table 23",
            source_file: "table_23.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "js",
            grade: 7,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 12.5,
            lower_um: -12.5,
            source_table: "Table 23",
            source_file: "table_23.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "js",
            grade: 5,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 16.0,
            lower_um: -16.0,
            source_table: "Table 23",
            source_file: "table_23.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "k",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 6.0,
            lower_um: 0.0,
            source_table: "Table 24",
            source_file: "table_24.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "k",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 15.0,
            lower_um: 2.0,
            source_table: "Table 24",
            source_file: "table_24.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "k",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 18.0,
            lower_um: 2.0,
            source_table: "Table 24",
            source_file: "table_24.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "k",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 44.0,
            lower_um: 0.0,
            source_table: "Table 24",
            source_file: "table_24.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "m",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 8.0,
            lower_um: 2.0,
            source_table: "Table 25",
            source_file: "table_25.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "m",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 21.0,
            lower_um: 8.0,
            source_table: "Table 25",
            source_file: "table_25.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "m",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 25.0,
            lower_um: 9.0,
            source_table: "Table 25",
            source_file: "table_25.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "m",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 70.0,
            lower_um: 26.0,
            source_table: "Table 25",
            source_file: "table_25.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "n",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 10.0,
            lower_um: 4.0,
            source_table: "Table 25",
            source_file: "table_25.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "n",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 28.0,
            lower_um: 15.0,
            source_table: "Table 25",
            source_file: "table_25.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "n",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 33.0,
            lower_um: 17.0,
            source_table: "Table 25",
            source_file: "table_25.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "p",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 12.0,
            lower_um: 6.0,
            source_table: "Table 26",
            source_file: "table_26.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "p",
            grade: 6,
            nominal_mm: 20.0,
            size_min: 18.0,
            size_max: 30.0,
            upper_um: 35.0,
            lower_um: 22.0,
            source_table: "Table 26",
            source_file: "table_26.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "p",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 42.0,
            lower_um: 26.0,
            source_table: "Table 26",
            source_file: "table_26.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "p",
            grade: 6,
            nominal_mm: 550.0,
            size_min: 500.0,
            size_max: 630.0,
            upper_um: 122.0,
            lower_um: 78.0,
            source_table: "Table 26",
            source_file: "table_26.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "r",
            grade: 6,
            nominal_mm: 3.0,
            size_min: 0.0,
            size_max: 3.0,
            upper_um: 16.0,
            lower_um: 10.0,
            source_table: "Table 27",
            source_file: "table_27.csv",
        },
        GoldenTolerance {
            feature: "shaft",
            zone: "r",
            grade: 6,
            nominal_mm: 42.0,
            size_min: 30.0,
            size_max: 50.0,
            upper_um: 50.0,
            lower_um: 34.0,
            source_table: "Table 27",
            source_file: "table_27.csv",
        },
    ]
}
