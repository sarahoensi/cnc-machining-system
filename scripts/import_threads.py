#!/usr/bin/env python3
import csv
import json
import sqlite3
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional

SCHEMA_SQL = """
PRAGMA foreign_keys = ON;

CREATE TABLE thread_specs(
  id TEXT PRIMARY KEY,
  thread_type TEXT NOT NULL CHECK(thread_type IN ('metric', 'unc', 'unf', 'bsp')),
  family TEXT NOT NULL,
  series TEXT NOT NULL,
  designation TEXT NOT NULL,
  display_name TEXT NOT NULL,
  nominal_pipe_size TEXT,
  nominal_diameter_in REAL,
  nominal_diameter_mm REAL,
  major_diameter_mm REAL NOT NULL,
  pitch_mm REAL NOT NULL,
  tpi REAL,
  profile_angle_deg REAL NOT NULL,
  pitch_diameter_mm REAL,
  minor_diameter_male_mm REAL,
  tap_drill_mm REAL NOT NULL,
  radial_thread_depth_mm REAL NOT NULL,
  is_default_pitch INTEGER NOT NULL CHECK(is_default_pitch IN (0, 1)),
  tap_drill_basis TEXT NOT NULL,
  depth_basis TEXT NOT NULL,
  standard_reference TEXT NOT NULL,
  source_url TEXT NOT NULL,
  profile_source_url TEXT,
  data_version TEXT NOT NULL,
  verification_status TEXT NOT NULL,
  source_file TEXT NOT NULL,
  UNIQUE(thread_type, designation, pitch_mm)
);

CREATE INDEX idx_thread_specs_options
ON thread_specs(thread_type, designation, pitch_mm);

CREATE INDEX idx_thread_specs_series
ON thread_specs(thread_type, series, designation);

CREATE TABLE metadata(
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
"""

REQUIRED_COLUMNS = {
    "id",
    "family",
    "series",
    "designation",
    "display_name",
    "pitch_mm",
    "profile_angle_deg",
    "tap_drill_mm",
    "radial_thread_depth_mm",
    "tap_drill_basis",
    "depth_basis",
    "standard_reference",
    "source_url",
    "data_version",
    "verification_status",
}

THREAD_TYPE_BY_FILE = {
    "threads_metric.csv": "metric",
    "threads_unc.csv": "unc",
    "threads_unf.csv": "unf",
    "threads_bsp_g.csv": "bsp",
}


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    csv_dir = repo_root / "src-tauri" / "data" / "threads" / "csv"
    db_path = repo_root / "src-tauri" / "data" / "threads.sqlite"

    try:
        import_database(csv_dir, db_path)
    except ImportError as err:
        print(f"Thread import failed: {err}", file=sys.stderr)
        return 1

    return 0


def import_database(csv_dir: Path, db_path: Path) -> None:
    if not csv_dir.exists():
        raise ImportError(f"CSV directory not found: {csv_dir}")

    manifest_path = csv_dir / "manifest.json"
    manifest = read_manifest(manifest_path)
    csv_files = expected_csv_files(csv_dir, manifest)

    rows: list[dict[str, object]] = []
    seen_ids: set[str] = set()

    for csv_file in csv_files:
        file_rows = read_thread_csv(csv_file)
        expected_rows = manifest_row_count(manifest, csv_file.name)
        if expected_rows is not None and expected_rows != len(file_rows):
            raise ImportError(
                f"{csv_file.name}: manifest expects {expected_rows} rows, found {len(file_rows)}"
            )

        for row in file_rows:
            row_id = str(row["id"])
            if row_id in seen_ids:
                raise ImportError(f"{csv_file.name}: duplicate id '{row_id}'")
            seen_ids.add(row_id)
            rows.append(row)

        print(f"{csv_file.name}: imported {len(file_rows)} rows")

    db_path.parent.mkdir(parents=True, exist_ok=True)
    if db_path.exists():
        db_path.unlink()

    conn = sqlite3.connect(db_path)
    try:
        conn.executescript(SCHEMA_SQL)
        insert_thread_rows(conn, rows)
        insert_metadata(conn, manifest, csv_files, len(rows))
        conn.commit()
    except Exception:
        conn.rollback()
        raise
    finally:
        conn.close()

    print(f"Total imported rows: {len(rows)}")
    print(f"Wrote database: {db_path}")


def read_manifest(path: Path) -> dict[str, object]:
    if not path.exists():
        raise ImportError(f"manifest.json not found: {path}")
    with path.open("r", encoding="utf-8-sig") as handle:
        return json.load(handle)


def expected_csv_files(csv_dir: Path, manifest: dict[str, object]) -> list[Path]:
    files = manifest.get("files")
    if not isinstance(files, list):
        raise ImportError("manifest.json must contain a files array")

    paths: list[Path] = []
    for entry in files:
        if not isinstance(entry, dict) or not isinstance(entry.get("file"), str):
            raise ImportError("manifest file entries must contain a file string")
        path = csv_dir / entry["file"]
        if not path.exists():
            raise ImportError(f"CSV file listed in manifest not found: {path}")
        paths.append(path)
    return paths


def manifest_row_count(manifest: dict[str, object], file_name: str) -> Optional[int]:
    files = manifest.get("files")
    if not isinstance(files, list):
        return None
    for entry in files:
        if isinstance(entry, dict) and entry.get("file") == file_name:
            rows = entry.get("rows")
            return int(rows) if rows is not None else None
    return None


def read_thread_csv(path: Path) -> list[dict[str, object]]:
    thread_type = THREAD_TYPE_BY_FILE.get(path.name)
    if thread_type is None:
        raise ImportError(f"{path.name}: no thread_type mapping exists")

    with path.open("r", encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        if reader.fieldnames is None:
            raise ImportError(f"{path.name}: missing header row")
        validate_header(path.name, reader.fieldnames)

        rows = [
            normalize_row(path.name, line_number, thread_type, row)
            for line_number, row in enumerate(reader, start=2)
            if any((value or "").strip() for value in row.values())
        ]

    if not rows:
        raise ImportError(f"{path.name}: file contains no data rows")
    return rows


def validate_header(file_name: str, headers: list[str]) -> None:
    missing = sorted(REQUIRED_COLUMNS - set(headers))
    if missing:
        raise ImportError(f"{file_name}: missing required columns: {', '.join(missing)}")

    if "major_diameter_mm" not in headers and "nominal_diameter_mm" not in headers:
        raise ImportError(
            f"{file_name}: expected major_diameter_mm or nominal_diameter_mm"
        )


def normalize_row(
    file_name: str,
    line_number: int,
    thread_type: str,
    row: dict[str, str],
) -> dict[str, object]:
    major_diameter_mm = parse_optional_float(row.get("major_diameter_mm"))
    nominal_diameter_mm = parse_optional_float(row.get("nominal_diameter_mm"))
    if major_diameter_mm is None:
        major_diameter_mm = nominal_diameter_mm
    if major_diameter_mm is None:
        raise ImportError(f"{file_name}:{line_number}: major diameter is required")

    result = {
        "id": required_text(file_name, line_number, row, "id"),
        "thread_type": thread_type,
        "family": required_text(file_name, line_number, row, "family"),
        "series": required_text(file_name, line_number, row, "series"),
        "designation": required_text(file_name, line_number, row, "designation"),
        "display_name": required_text(file_name, line_number, row, "display_name"),
        "nominal_pipe_size": optional_text(row.get("nominal_pipe_size")),
        "nominal_diameter_in": parse_optional_float(row.get("nominal_diameter_in")),
        "nominal_diameter_mm": nominal_diameter_mm,
        "major_diameter_mm": major_diameter_mm,
        "pitch_mm": required_float(file_name, line_number, row, "pitch_mm"),
        "tpi": parse_optional_float(row.get("tpi")),
        "profile_angle_deg": required_float(file_name, line_number, row, "profile_angle_deg"),
        "pitch_diameter_mm": parse_optional_float(row.get("pitch_diameter_mm")),
        "minor_diameter_male_mm": parse_optional_float(row.get("minor_diameter_male_mm")),
        "tap_drill_mm": required_float(file_name, line_number, row, "tap_drill_mm"),
        "radial_thread_depth_mm": required_float(
            file_name,
            line_number,
            row,
            "radial_thread_depth_mm",
        ),
        "is_default_pitch": parse_optional_bool(row.get("is_default_pitch"), default=1),
        "tap_drill_basis": required_text(file_name, line_number, row, "tap_drill_basis"),
        "depth_basis": required_text(file_name, line_number, row, "depth_basis"),
        "standard_reference": required_text(file_name, line_number, row, "standard_reference"),
        "source_url": required_text(file_name, line_number, row, "source_url"),
        "profile_source_url": optional_text(row.get("profile_source_url")),
        "data_version": required_text(file_name, line_number, row, "data_version"),
        "verification_status": required_text(file_name, line_number, row, "verification_status"),
        "source_file": file_name,
    }

    if float(result["pitch_mm"]) <= 0:
        raise ImportError(f"{file_name}:{line_number}: pitch_mm must be greater than zero")
    if float(result["tap_drill_mm"]) <= 0:
        raise ImportError(f"{file_name}:{line_number}: tap_drill_mm must be greater than zero")
    if float(result["radial_thread_depth_mm"]) <= 0:
        raise ImportError(
            f"{file_name}:{line_number}: radial_thread_depth_mm must be greater than zero"
        )

    return result


def required_text(
    file_name: str,
    line_number: int,
    row: dict[str, str],
    column: str,
) -> str:
    value = optional_text(row.get(column))
    if value is None:
        raise ImportError(f"{file_name}:{line_number}: {column} is required")
    return value


def optional_text(value: Optional[str]) -> Optional[str]:
    stripped = (value or "").strip()
    return stripped or None


def required_float(
    file_name: str,
    line_number: int,
    row: dict[str, str],
    column: str,
) -> float:
    value = parse_optional_float(row.get(column))
    if value is None:
        raise ImportError(f"{file_name}:{line_number}: {column} is required")
    return value


def parse_optional_float(value: Optional[str]) -> Optional[float]:
    stripped = (value or "").strip().replace(",", ".")
    if not stripped:
        return None
    return float(stripped)


def parse_optional_bool(value: Optional[str], default: int) -> int:
    if optional_text(value) is None:
        return default

    return parse_bool(value or "")


def parse_bool(value: str) -> int:
    normalized = value.strip().lower()
    if normalized == "true":
        return 1
    if normalized == "false":
        return 0
    raise ImportError("is_default_pitch must be true or false")


def insert_thread_rows(conn: sqlite3.Connection, rows: list[dict[str, object]]) -> None:
    columns = [
        "id",
        "thread_type",
        "family",
        "series",
        "designation",
        "display_name",
        "nominal_pipe_size",
        "nominal_diameter_in",
        "nominal_diameter_mm",
        "major_diameter_mm",
        "pitch_mm",
        "tpi",
        "profile_angle_deg",
        "pitch_diameter_mm",
        "minor_diameter_male_mm",
        "tap_drill_mm",
        "radial_thread_depth_mm",
        "is_default_pitch",
        "tap_drill_basis",
        "depth_basis",
        "standard_reference",
        "source_url",
        "profile_source_url",
        "data_version",
        "verification_status",
        "source_file",
    ]
    placeholders = ", ".join("?" for _ in columns)
    conn.executemany(
        f"INSERT INTO thread_specs({', '.join(columns)}) VALUES ({placeholders})",
        [tuple(row[column] for column in columns) for row in rows],
    )


def insert_metadata(
    conn: sqlite3.Connection,
    manifest: dict[str, object],
    csv_files: list[Path],
    row_count: int,
) -> None:
    metadata = {
        "data_version": str(manifest.get("data_version", "")),
        "generated_from": "local_thread_csv_files",
        "source_files": ",".join(path.name for path in csv_files),
        "row_count": str(row_count),
        "generated_at": datetime.now(timezone.utc).isoformat(),
    }
    conn.executemany(
        "INSERT INTO metadata(key, value) VALUES (?, ?)",
        sorted(metadata.items()),
    )


class ImportError(Exception):
    pass


if __name__ == "__main__":
    raise SystemExit(main())
