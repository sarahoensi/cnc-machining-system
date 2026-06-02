#!/usr/bin/env python3
import csv
import sqlite3
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path

EXPECTED_HEADER = [
    "feature",
    "zone",
    "grade",
    "size_min",
    "size_max",
    "upper_um",
    "lower_um",
    "source_table",
]

SCHEMA_SQL = """
PRAGMA foreign_keys = ON;

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
"""


@dataclass(frozen=True)
class ToleranceRow:
    feature: str
    zone: str
    grade: int
    size_min: float
    size_max: float
    upper_um: float
    lower_um: float
    source_table: str
    source_file: str

    @property
    def duplicate_key(self) -> tuple[str, str, int, float, float]:
        return (self.feature, self.zone, self.grade, self.size_min, self.size_max)


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    csv_dir = repo_root / "data" / "iso286" / "normalized"
    db_path = repo_root / "src-tauri" / "data" / "iso286.sqlite"

    try:
        import_database(csv_dir, db_path)
    except ImportError as err:
        print(f"ISO 286 import failed: {err}", file=sys.stderr)
        return 1

    return 0


def import_database(csv_dir: Path, db_path: Path) -> None:
    if not csv_dir.exists():
        raise ImportError(
            f"CSV directory '{csv_dir}' does not exist. Create it and add normalized ISO 286 CSV files first."
        )

    csv_files = sorted(path for path in csv_dir.glob("*.csv") if path.is_file())
    if not csv_files:
        raise ImportError(
            f"CSV directory '{csv_dir}' contains no CSV files. ISO data is not included yet."
        )

    all_rows: list[ToleranceRow] = []
    seen: set[tuple[str, str, int, float, float]] = set()
    file_counts: dict[str, int] = {}

    for csv_file in csv_files:
        rows = read_csv_file(csv_file)
        for row in rows:
            if row.duplicate_key in seen:
                raise ImportError(
                    f"{csv_file.name}: duplicate tolerance row for "
                    f"{row.feature} {row.zone}{row.grade} {row.size_min:g}-{row.size_max:g}"
                )
            seen.add(row.duplicate_key)
        all_rows.extend(rows)
        file_counts[csv_file.name] = len(rows)

    db_path.parent.mkdir(parents=True, exist_ok=True)
    if db_path.exists():
        db_path.unlink()

    conn = sqlite3.connect(db_path)
    try:
        conn.executescript(SCHEMA_SQL)
        insert_rows(conn, all_rows)
        insert_metadata(conn)
        conn.commit()
    except Exception:
        conn.rollback()
        raise
    finally:
        conn.close()

    for name, count in file_counts.items():
        print(f"{name}: imported {count} rows")
    print(f"Total imported rows: {len(all_rows)}")
    print(f"Wrote database: {db_path}")


def read_csv_file(path: Path) -> list[ToleranceRow]:
    rows: list[ToleranceRow] = []
    with path.open("r", encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        header = [name.strip() for name in reader.fieldnames or []]
        if header != EXPECTED_HEADER:
            raise ImportError(
                f"{path.name}: invalid header. Expected '{','.join(EXPECTED_HEADER)}', "
                f"found '{','.join(header)}'"
            )

        for line_number, raw_row in enumerate(reader, start=2):
            if not raw_row or all((value or "").strip() == "" for value in raw_row.values()):
                continue
            rows.append(parse_row(path.name, line_number, raw_row))

    if not rows:
        raise ImportError(f"{path.name}: file contains no data rows")

    return rows


def parse_row(file_name: str, line_number: int, raw: dict[str, str]) -> ToleranceRow:
    feature = read_text(raw, "feature")
    if feature not in {"hole", "shaft"}:
        raise ImportError(f"{file_name}:{line_number}: feature must be 'hole' or 'shaft'")

    zone = read_text(raw, "zone")
    if not zone:
        raise ImportError(f"{file_name}:{line_number}: zone is required")

    grade_text = read_text(raw, "grade")
    try:
        grade = int(grade_text)
    except ValueError as err:
        raise ImportError(f"{file_name}:{line_number}: grade must be an integer") from err

    size_min = parse_number(file_name, line_number, "size_min", raw["size_min"])
    size_max = parse_number(file_name, line_number, "size_max", raw["size_max"])
    upper_um = parse_number(file_name, line_number, "upper_um", raw["upper_um"])
    lower_um = parse_number(file_name, line_number, "lower_um", raw["lower_um"])

    if size_min >= size_max:
        raise ImportError(
            f"{file_name}:{line_number}: size_min must be less than size_max ({size_min:g} >= {size_max:g})"
        )
    if upper_um < lower_um:
        raise ImportError(
            f"{file_name}:{line_number}: upper_um must be greater than or equal to lower_um "
            f"({upper_um:g} < {lower_um:g})"
        )

    return ToleranceRow(
        feature=feature,
        zone=zone,
        grade=grade,
        size_min=size_min,
        size_max=size_max,
        upper_um=upper_um,
        lower_um=lower_um,
        source_table=read_text(raw, "source_table"),
        source_file=file_name,
    )


def read_text(row: dict[str, str], column: str) -> str:
    return (row[column] or "").strip()


def parse_number(file_name: str, line_number: int, column: str, value: str) -> float:
    normalized = (value or "").strip().replace("−", "-").replace(",", ".")
    if not normalized:
        raise ImportError(f"{file_name}:{line_number}: {column} is required")
    try:
        return float(normalized)
    except ValueError as err:
        raise ImportError(
            f"{file_name}:{line_number}: {column} has invalid numeric value '{value}'"
        ) from err


def insert_rows(conn: sqlite3.Connection, rows: list[ToleranceRow]) -> None:
    conn.executemany(
        """
        INSERT INTO tolerance_zones(
          feature, zone, grade, size_min, size_max, upper_um, lower_um, source_table, source_file
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
        [
            (
                row.feature,
                row.zone,
                row.grade,
                row.size_min,
                row.size_max,
                row.upper_um,
                row.lower_um,
                row.source_table,
                row.source_file,
            )
            for row in rows
        ],
    )


def insert_metadata(conn: sqlite3.Connection) -> None:
    metadata = {
        "standard": "ISO 286-2:2010",
        "import_model": "full_table_lookup",
        "generated_from": "local_csv_files",
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
