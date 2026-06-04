#!/usr/bin/env python3
import csv
import re
import sqlite3
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional, Sequence, Tuple, Union

EXPECTED_NORMALIZED_HEADER = [
    "feature",
    "zone",
    "grade",
    "size_min",
    "size_max",
    "upper_um",
    "lower_um",
    "source_table",
]

EXPECTED_IT_HEADERS = ["IT01", "IT0"] + [f"IT{i}" for i in range(1, 19)]
REQUIRED_HOLE_ZONES = [
    "D",
    "E",
    "F",
    "G",
    "H",
    "J",
    "JS",
    "K",
    "M",
    "N",
    "P",
    "R",
    "S",
    "T",
    "U",
    "V",
    "X",
    "Y",
    "Z",
    "ZA",
    "ZB",
    "ZC",
]
REQUIRED_SHAFT_ZONES = ["f", "g", "h", "js", "k", "m", "n", "p", "r"]

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

CREATE TABLE iso_it_grades(
  id INTEGER PRIMARY KEY,
  size_min REAL NOT NULL,
  size_max REAL NOT NULL,
  IT01 REAL,
  IT0 REAL,
  IT1 REAL,
  IT2 REAL,
  IT3 REAL,
  IT4 REAL,
  IT5 REAL,
  IT6 REAL,
  IT7 REAL,
  IT8 REAL,
  IT9 REAL,
  IT10 REAL,
  IT11 REAL,
  IT12 REAL,
  IT13 REAL,
  IT14 REAL,
  IT15 REAL,
  IT16 REAL,
  IT17 REAL,
  IT18 REAL,
  CHECK(size_min < size_max),
  UNIQUE(size_min, size_max)
);

CREATE INDEX idx_it_grades_range
ON iso_it_grades(size_min, size_max);

CREATE TABLE metadata(
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
"""

TABLE_NUMBER_PATTERN = re.compile(r"table[_\-]?0*(\d+)", re.IGNORECASE)
TOLERANCE_ES_PATTERN = re.compile(r"^([A-Za-z]+)(\d+)_ES(?:_um)?$", re.IGNORECASE)
TOLERANCE_EI_PATTERN = re.compile(r"^([A-Za-z]+)(\d+)_EI(?:_um)?$", re.IGNORECASE)


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
    def duplicate_key(self) -> Tuple[str, str, int, float, float]:
        return (self.feature, self.zone, self.grade, self.size_min, self.size_max)


@dataclass(frozen=True)
class ItGradeRow:
    size_min: float
    size_max: float
    values: dict[str, Optional[float]]
    source_file: str


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    source_csv_dir = repo_root / "src-tauri" / "data" / "csv"
    normalized_dir = repo_root / "data" / "iso286" / "normalized"
    db_path = repo_root / "src-tauri" / "data" / "iso286.sqlite"

    csv_dirs = [path for path in (source_csv_dir, normalized_dir) if path.exists()]
    if not csv_dirs:
        raise ImportError(
            f"No ISO 286 CSV source directory found. Put raw tables in '{source_csv_dir}' or normalized files in '{normalized_dir}'."
        )

    try:
        import_database(csv_dirs, db_path, validate_required_zones=True)
    except ImportError as err:
        print(f"ISO 286 import failed: {err}", file=sys.stderr)
        return 1

    return 0


def import_database(
    csv_dirs: Union[Path, Sequence[Path]],
    db_path: Path,
    validate_required_zones: bool = False,
) -> None:
    if isinstance(csv_dirs, Path):
        csv_dirs = [csv_dirs]

    csv_files = sorted(
        path
        for csv_dir in csv_dirs
        for path in csv_dir.glob("*.csv")
        if path.is_file()
    )
    if not csv_files:
        raise ImportError(
            f"No CSV files found in {[str(path) for path in csv_dirs]}."
        )

    all_tolerance_rows: list[ToleranceRow] = []
    all_it_rows: list[ItGradeRow] = []
    seen_tolerance_keys: set[Tuple[str, str, int, float, float]] = set()
    file_counts: dict[str, int] = {}

    for csv_file in csv_files:
        row_type, rows = read_csv_file(csv_file)
        if row_type == "tolerance":
            for row in rows:
                if row.duplicate_key in seen_tolerance_keys:
                    raise ImportError(
                        f"{csv_file.name}: duplicate tolerance row for "
                        f"{row.feature} {row.zone}{row.grade} {row.size_min:g}-{row.size_max:g}"
                    )
                seen_tolerance_keys.add(row.duplicate_key)
            all_tolerance_rows.extend(rows)
        else:
            all_it_rows.extend(rows)

        file_counts[csv_file.name] = len(rows)

    if validate_required_zones:
        validate_required_tolerance_zones(all_tolerance_rows)

    db_path.parent.mkdir(parents=True, exist_ok=True)
    if db_path.exists():
        db_path.unlink()

    conn = sqlite3.connect(db_path)
    try:
        conn.executescript(SCHEMA_SQL)
        insert_tolerance_rows(conn, all_tolerance_rows)
        insert_it_grade_rows(conn, all_it_rows)
        insert_metadata(conn, csv_files)
        conn.commit()
    except Exception:
        conn.rollback()
        raise
    finally:
        conn.close()

    for name, count in file_counts.items():
        print(f"{name}: imported {count} rows")
    print(f"Total imported rows: {len(all_tolerance_rows) + len(all_it_rows)}")
    print(f"Wrote database: {db_path}")


def read_csv_file(path: Path) -> Tuple[str, Sequence[object]]:
    with path.open("r", encoding="utf-8-sig", newline="") as handle:
        dialect = detect_csv_dialect(handle)
        raw_rows = list(csv.reader(handle, dialect=dialect))
        header_index = find_header_row(path.name, raw_rows)
        if header_index is None:
            raise ImportError(f"{path.name}: missing header row")

        header = [name.strip() for name in raw_rows[header_index]]
        rows = rows_as_dicts(header, raw_rows[header_index + 1 :])
        stem = path.stem.lower()

        if is_table_1_file(stem):
            return "it_grades", read_it_grade_csv(path.name, header, rows)
        if is_raw_tolerance_table_file(stem):
            return "tolerance", read_iso286_tolerance_csv(path.name, header, rows)
        if header == EXPECTED_NORMALIZED_HEADER:
            return "tolerance", read_normalized_csv(path.name, rows)

        raise ImportError(
            f"{path.name}: invalid ISO 286 CSV file name or header. "
            f"Expected table_*.csv raw ISO 286 header or normalized ISO 286 header."
        )


def detect_csv_dialect(handle: object) -> csv.Dialect:
    sample = handle.read(4096)
    handle.seek(0)
    try:
        dialect = csv.Sniffer().sniff(sample, delimiters=";," )
    except csv.Error:
        dialect = csv.excel
    dialect.skipinitialspace = True
    return dialect


def find_header_row(file_name: str, rows: list[list[str]]) -> Optional[int]:
    for index, row in enumerate(rows):
        header = [value.strip() for value in row]
        if header == EXPECTED_NORMALIZED_HEADER:
            return index
        if len(header) >= 2 and is_size_column(header[0]) and is_upper_bound_column(header[1]):
            return index
    return None


def is_size_column(value: str) -> bool:
    return value.lower().startswith("above")


def is_upper_bound_column(value: str) -> bool:
    return value.lower().startswith("up")


def rows_as_dicts(header: list[str], rows: list[list[str]]) -> list[dict[str, str]]:
    result: list[dict[str, str]] = []
    for row in rows:
        padded = row + [""] * max(0, len(header) - len(row))
        result.append({column: padded[index].strip() for index, column in enumerate(header)})
    return result


def is_table_1_file(stem: str) -> bool:
    return bool(re.fullmatch(r"table[_\-]?0*1", stem))


def is_raw_tolerance_table_file(stem: str) -> bool:
    match = re.fullmatch(r"table[_\-]?0*(\d+)", stem)
    return bool(match and int(match.group(1)) != 1)


def read_it_grade_csv(file_name: str, header: list[str], reader: Sequence[dict[str, str]]) -> list[ItGradeRow]:
    if len(header) != 22:
        raise ImportError(
            f"{file_name}: table 1 CSV must contain 22 columns, found {len(header)}"
        )
    if is_size_column(header[0]) is False or is_upper_bound_column(header[1]) is False:
        raise ImportError(
            f"{file_name}: invalid table 1 header. First columns must be 'Above' and 'Up_to_and_including'"
        )
    if [name.upper() for name in header[2:]] != EXPECTED_IT_HEADERS:
        raise ImportError(
            f"{file_name}: invalid table 1 header. Expected IT grade columns {EXPECTED_IT_HEADERS}"
        )

    rows: list[ItGradeRow] = []
    for line_number, raw_row in enumerate(reader, start=2):
        if not raw_row or all((value or "").strip() == "" for value in raw_row.values()):
            continue
        if is_note_row(raw_row, header[0]):
            continue

        size_min = parse_number(file_name, line_number, header[0], raw_row[header[0]])
        size_max = parse_number(file_name, line_number, header[1], raw_row[header[1]])
        if size_min >= size_max:
            raise ImportError(
                f"{file_name}:{line_number}: size_min must be less than size_max ({size_min:g} >= {size_max:g})"
            )

        values: dict[str, Optional[float]] = {}
        for column in header[2:]:
            raw_value = (raw_row[column] or "").strip()
            if raw_value == "":
                values[column.upper()] = None
            else:
                values[column.upper()] = parse_number(file_name, line_number, column, raw_value)

        rows.append(ItGradeRow(size_min=size_min, size_max=size_max, values=values, source_file=file_name))

    if not rows:
        raise ImportError(f"{file_name}: file contains no data rows")
    return rows


def read_iso286_tolerance_csv(file_name: str, header: list[str], reader: Sequence[dict[str, str]]) -> list[ToleranceRow]:
    if len(header) < 4:
        raise ImportError(f"{file_name}: raw tolerance CSV must contain at least 4 header columns")
    if is_size_column(header[0]) is False or is_upper_bound_column(header[1]) is False:
        raise ImportError(
            f"{file_name}: invalid raw tolerance header. First columns must be 'Above' and 'Up_to_and_including'"
        )

    column_pairs = parse_tolerance_columns(file_name, header[2:])
    source_table = source_table_from_file_name(file_name)
    rows: list[ToleranceRow] = []

    for line_number, raw_row in enumerate(reader, start=2):
        if not raw_row or all((value or "").strip() == "" for value in raw_row.values()):
            continue
        if is_note_row(raw_row, header[0]):
            continue

        size_min = parse_number(file_name, line_number, header[0], raw_row[header[0]])
        size_max = parse_number(file_name, line_number, header[1], raw_row[header[1]])
        if size_min >= size_max:
            raise ImportError(
                f"{file_name}:{line_number}: size_min must be less than size_max ({size_min:g} >= {size_max:g})"
            )

        for zone, grade, upper_column, lower_column in column_pairs:
            upper_text = (raw_row.get(upper_column) or "").strip()
            lower_text = (raw_row.get(lower_column) or "").strip()
            if upper_text == "" and lower_text == "":
                continue
            if upper_text == "" or lower_text == "":
                raise ImportError(
                    f"{file_name}:{line_number}: both {upper_column} and {lower_column} must be present"
                )

            upper_um = parse_number(file_name, line_number, upper_column, upper_text)
            lower_um = parse_number(file_name, line_number, lower_column, lower_text)
            feature = feature_from_zone(zone)
            rows.append(
                ToleranceRow(
                    feature=feature,
                    zone=zone,
                    grade=grade,
                    size_min=size_min,
                    size_max=size_max,
                    upper_um=upper_um,
                    lower_um=lower_um,
                    source_table=source_table,
                    source_file=file_name,
                )
            )

    if not rows:
        raise ImportError(f"{file_name}: file contains no data rows")
    return rows


def parse_tolerance_columns(file_name: str, columns: list[str]) -> list[Tuple[str, int, str, str]]:
    lower_to_name = {name.strip().lower(): name.strip() for name in columns}
    pairs: list[Tuple[str, int, str, str]] = []
    expected_lower_keys: set[str] = set()

    for name in columns:
        stripped = name.strip()
        match = TOLERANCE_ES_PATTERN.fullmatch(stripped)
        if not match:
            continue
        zone = match.group(1)
        grade = int(match.group(2))
        upper_column = stripped
        lower_keys = [f"{zone}{grade}_EI_um", f"{zone}{grade}_EI"]
        lower_column = next(
            (lower_to_name[key.lower()] for key in lower_keys if key.lower() in lower_to_name),
            None,
        )
        if lower_column is None:
            raise ImportError(
                f"{file_name}: missing matching {lower_keys[0]} or {lower_keys[1]} for {upper_column}"
            )
        pairs.append((zone, grade, upper_column, lower_column))
        expected_lower_keys.add(lower_column.lower())

    actual_ei_keys = {
        name.strip().lower()
        for name in columns
        if TOLERANCE_EI_PATTERN.fullmatch(name.strip())
    }
    if actual_ei_keys != expected_lower_keys:
        missing = actual_ei_keys - expected_lower_keys
        extra = expected_lower_keys - actual_ei_keys
        message = f"{file_name}: invalid raw tolerance columns."
        if missing:
            message += f" Unmatched EI columns: {sorted(missing)}."
        if extra:
            message += f" Missing EI columns: {sorted(extra)}."
        raise ImportError(message)

    if not pairs:
        raise ImportError(f"{file_name}: no tolerance zone columns found")

    return pairs


def source_table_from_file_name(file_name: str) -> str:
    match = TABLE_NUMBER_PATTERN.search(file_name)
    if not match:
        return file_name
    return f"Table {match.group(1)}"


def feature_from_zone(zone: str) -> str:
    if zone.islower():
        return "shaft"
    return "hole"


def read_normalized_csv(file_name: str, reader: Sequence[dict[str, str]]) -> list[ToleranceRow]:
    rows: list[ToleranceRow] = []
    for line_number, raw_row in enumerate(reader, start=2):
        if not raw_row or all((value or "").strip() == "" for value in raw_row.values()):
            continue
        rows.append(parse_normalized_row(file_name, line_number, raw_row))

    if not rows:
        raise ImportError(f"{file_name}: file contains no data rows")
    return rows


def read_text(row: dict[str, str], column: str) -> str:
    return (row[column] or "").strip()


def is_note_row(row: dict[str, str], first_column: str) -> bool:
    return read_text(row, first_column).lower().startswith("note:")


def parse_normalized_row(file_name: str, line_number: int, row: dict[str, str]) -> ToleranceRow:
    feature = read_text(row, "feature")
    if feature not in {"hole", "shaft"}:
        raise ImportError(
            f"{file_name}:{line_number}: feature must be 'hole' or 'shaft'"
        )

    zone = read_text(row, "zone")
    if not zone or not zone.isalpha():
        raise ImportError(f"{file_name}:{line_number}: zone must contain letters")

    grade = int(parse_number(file_name, line_number, "grade", read_text(row, "grade")))
    size_min = parse_number(file_name, line_number, "size_min", read_text(row, "size_min"))
    size_max = parse_number(file_name, line_number, "size_max", read_text(row, "size_max"))
    upper_um = parse_number(file_name, line_number, "upper_um", read_text(row, "upper_um"))
    lower_um = parse_number(file_name, line_number, "lower_um", read_text(row, "lower_um"))

    if size_min >= size_max:
        raise ImportError(
            f"{file_name}:{line_number}: size_min must be less than size_max ({size_min:g} >= {size_max:g})"
        )
    if upper_um < lower_um:
        raise ImportError(
            f"{file_name}:{line_number}: upper_um must be greater than or equal to lower_um"
        )

    return ToleranceRow(
        feature=feature,
        zone=zone,
        grade=grade,
        size_min=size_min,
        size_max=size_max,
        upper_um=upper_um,
        lower_um=lower_um,
        source_table=read_text(row, "source_table"),
        source_file=file_name,
    )


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


def insert_tolerance_rows(conn: sqlite3.Connection, rows: list[ToleranceRow]) -> None:
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


def insert_it_grade_rows(conn: sqlite3.Connection, rows: list[ItGradeRow]) -> None:
    if not rows:
        return
    columns = ["size_min", "size_max"] + EXPECTED_IT_HEADERS
    placeholders = ", ".join("?" for _ in columns)
    sql = f"INSERT INTO iso_it_grades({', '.join(columns)}) VALUES ({placeholders})"
    conn.executemany(
        sql,
        [
            tuple(
                [row.size_min, row.size_max]
                + [row.values.get(column) for column in EXPECTED_IT_HEADERS]
            )
            for row in rows
        ],
    )


def insert_metadata(conn: sqlite3.Connection, csv_files: Sequence[Path]) -> None:
    metadata = {
        "standard": "ISO 286-2:2010",
        "import_model": "full_table_lookup",
        "generated_from": "local_csv_files",
        "imported_tables": imported_tables_value(csv_files),
        "generated_at": datetime.now(timezone.utc).isoformat(),
    }
    conn.executemany(
        "INSERT INTO metadata(key, value) VALUES (?, ?)",
        sorted(metadata.items()),
    )


def imported_tables_value(csv_files: Sequence[Path]) -> str:
    table_numbers: list[int] = []
    for csv_file in csv_files:
        match = TABLE_NUMBER_PATTERN.search(csv_file.name)
        if match:
            table_numbers.append(int(match.group(1)))
    return ",".join(str(number) for number in sorted(set(table_numbers)))


def validate_required_tolerance_zones(rows: Sequence[ToleranceRow]) -> None:
    available_holes = {row.zone for row in rows if row.feature == "hole"}
    available_shafts = {row.zone for row in rows if row.feature == "shaft"}
    missing_holes = [zone for zone in REQUIRED_HOLE_ZONES if zone not in available_holes]
    missing_shafts = [zone for zone in REQUIRED_SHAFT_ZONES if zone not in available_shafts]

    if missing_holes or missing_shafts:
        parts: list[str] = []
        if missing_holes:
            parts.append(f"missing hole zones: {', '.join(missing_holes)}")
        if missing_shafts:
            parts.append(f"missing shaft zones: {', '.join(missing_shafts)}")
        raise ImportError(
            "Required ISO 286 tolerance zones are missing (" + "; ".join(parts) + ")"
        )


class ImportError(Exception):
    pass


if __name__ == "__main__":
    raise SystemExit(main())
