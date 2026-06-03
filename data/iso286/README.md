# ISO 286 Data Import

ISO 286 data is not included in this repository yet. The app must not be used for CNC production until the imported data has been manually verified against the licensed source standard.

## Directory Layout

Place raw ISO 286 CSV tables here:

```text
src-tauri/data/csv/
```

Normalized ISO 286 CSV files may also be placed in:

```text
data/iso286/normalized/
```

Keep raw source exports or working files here:

```text
data/iso286/raw_tables/
```

The import script generates:

```text
src-tauri/data/iso286.sqlite
```

## CSV Format

Each normalized CSV file must use this exact header:

```csv
feature,zone,grade,size_min,size_max,upper_um,lower_um,source_table
```

Example rows:

```csv
hole,H,7,30,50,25,0,Table 6
shaft,g,6,30,50,-9,-25,Table 21
```

Fields:

- `feature`: `hole` or `shaft`
- `zone`: tolerance zone, for example `H`, `JS`, `g`, or `js`
- `grade`: integer tolerance grade
- `size_min`: lower nominal size bound in mm
- `size_max`: upper nominal size bound in mm
- `upper_um`: upper deviation in micrometres
- `lower_um`: lower deviation in micrometres
- `source_table`: source table label used for traceability

The runtime lookup interval rule is:

```text
nominal_mm > size_min AND nominal_mm <= size_max
```

## Import

From the repository root:

```powershell
python scripts/import_iso286.py
```

The script reads every `.csv` file in `data/iso286/normalized/`, validates the rows, deletes any previous `src-tauri/data/iso286.sqlite`, creates the SQLite schema, and imports the data with `source_file` set from the CSV filename.

The importer writes metadata:

- `standard = ISO 286-2:2010`
- `import_model = full_table_lookup`
- `generated_from = local_csv_files`
- `generated_at = timestamp`
