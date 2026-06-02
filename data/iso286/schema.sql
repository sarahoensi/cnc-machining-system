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
