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
