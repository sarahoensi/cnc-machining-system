import sqlite3
import tempfile
import unittest
from pathlib import Path

from import_iso286 import import_database

TABLE_1_CSV = """Above;Up_to_and_including;IT01;IT0;IT1;IT2;IT3;IT4;IT5;IT6;IT7;IT8;IT9;IT10;IT11;IT12;IT13;IT14;IT15;IT16;IT17;IT18
0;3;0,3;0,5;0,8;1,2;2;3;4;6;10;14;25;40;60;0,1;0,14;0,25;0,4;0,6;1;1,4
"""

TABLE_2_CSV = """Above_mm;Up_to_and_including_mm;A9_ES_um;A9_EI_um;B8_ES_um;B8_EI_um
0;3;295;270;154;140
"""

TABLE_3_CSV = """Above_mm;Up_to_and_including_mm;D6_ES_um;D6_EI_um;E5_ES_um;E5_EI_um
0;3;26;20;18;14
"""

TABLE_4_CSV = """Above_mm;Up_to_and_including_mm;F3_ES_um;F3_EI_um;F4_ES_um;F4_EI_um
0;3;8;6;9;6
"""

TABLE_22_CSV = """Above_mm;Up_to_and_including_mm;h6_es_um;h6_ei_um;js6_es_um;js6_ei_um
0;3;0;-6;3;-3
3;6;0;-8;4;-4
"""

TABLE_23_CSV = """Above_mm;Up_to_and_including_mm;JS6_ES_um;JS6_EI_um;H7_ES_um;H7_EI_um
0;3;3;-3;10;0
3;6;4;-4;12;0
"""

TABLE_26_CSV = """Above_mm;Up_to_and_including_mm;p3_es_um;p3_ei_um;p4_es_um;p4_ei_um
0;3;8;6;9;6
3;6;14,5;12;16;12
6;10;;;;
"""

REQUIRED_ZONES_CSV = """feature,zone,grade,size_min,size_max,upper_um,lower_um,source_table
hole,D,6,0,3,26,20,fixture
hole,E,6,0,3,20,14,fixture
hole,F,6,0,3,10,4,fixture
hole,G,6,0,3,8,2,fixture
hole,H,7,0,3,10,0,fixture
hole,J,6,0,3,6,-4,fixture
hole,JS,7,0,3,5,-5,fixture
hole,K,6,0,3,0,-6,fixture
hole,M,6,0,3,-2,-8,fixture
hole,N,6,0,3,-4,-10,fixture
hole,P,6,0,3,-6,-12,fixture
hole,R,6,0,3,-10,-16,fixture
hole,S,6,0,3,-14,-20,fixture
hole,T,6,0,3,-18,-24,fixture
hole,U,6,0,3,-22,-28,fixture
hole,V,6,0,3,-26,-32,fixture
hole,X,6,0,3,-30,-36,fixture
hole,Y,6,0,3,-34,-40,fixture
hole,Z,6,0,3,-38,-44,fixture
hole,ZA,6,0,3,-42,-48,fixture
hole,ZB,6,0,3,-46,-52,fixture
hole,ZC,6,0,3,-50,-56,fixture
shaft,f,6,0,3,-6,-12,fixture
shaft,g,6,0,3,-2,-8,fixture
shaft,h,6,0,3,0,-6,fixture
shaft,js,6,0,3,3,-3,fixture
shaft,k,6,0,3,6,0,fixture
shaft,m,6,0,3,8,2,fixture
shaft,n,6,0,3,10,4,fixture
shaft,p,6,0,3,16,10,fixture
shaft,r,6,0,3,20,14,fixture
"""

MISSING_REQUIRED_ZONES_CSV = REQUIRED_ZONES_CSV.replace(
    "hole,Y,6,0,3,-34,-40,fixture\n",
    "hole,YC,6,0,3,-34,-40,fixture\n",
)

INCOMPLETE_PAIR_CSV = """Above_mm;Up_to_and_including_mm;p3_es_um;p3_ei_um
0;3;8;
"""


class ImportIso286Test(unittest.TestCase):
    def test_imports_it_grades_and_raw_tolerance_tables(self):
        with tempfile.TemporaryDirectory() as tmp_dir:
            repo_root = Path(tmp_dir)
            csv_dir = repo_root / "src-tauri" / "data" / "csv"
            csv_dir.mkdir(parents=True)
            (csv_dir / "table_1.csv").write_text(TABLE_1_CSV, encoding="utf-8")
            (csv_dir / "table_2.csv").write_text(TABLE_2_CSV, encoding="utf-8")
            (csv_dir / "table_3.csv").write_text(TABLE_3_CSV, encoding="utf-8")
            (csv_dir / "table_4.csv").write_text(TABLE_4_CSV, encoding="utf-8")
            (csv_dir / "table_22.csv").write_text(TABLE_22_CSV, encoding="utf-8")
            (csv_dir / "table_23.csv").write_text(TABLE_23_CSV, encoding="utf-8")
            (csv_dir / "table_26.csv").write_text(TABLE_26_CSV, encoding="utf-8")

            db_path = repo_root / "src-tauri" / "data" / "iso286.sqlite"
            import_database(csv_dir, db_path)

            conn = sqlite3.connect(db_path)
            try:
                it_count = conn.execute("SELECT COUNT(*) FROM iso_it_grades").fetchone()[0]
                tolerance_count = conn.execute("SELECT COUNT(*) FROM tolerance_zones").fetchone()[0]
                self.assertEqual(it_count, 1)
                self.assertEqual(tolerance_count, 18)

                row = conn.execute(
                    "SELECT feature, upper_um, lower_um FROM tolerance_zones WHERE zone = 'D' AND grade = 6"
                ).fetchone()
                self.assertIsNotNone(row)
                self.assertEqual(row, ("hole", 26.0, 20.0))

                shaft_row = conn.execute(
                    "SELECT feature, upper_um, lower_um, source_table FROM tolerance_zones WHERE zone = 'h' AND grade = 6 AND size_max = 6"
                ).fetchone()
                self.assertEqual(shaft_row, ("shaft", 0.0, -8.0, "Table 22"))

                hole_row = conn.execute(
                    "SELECT feature, upper_um, lower_um FROM tolerance_zones WHERE zone = 'JS' AND grade = 6 AND size_max = 6"
                ).fetchone()
                self.assertEqual(hole_row, ("hole", 4.0, -4.0))

                decimal_row = conn.execute(
                    "SELECT feature, upper_um, lower_um FROM tolerance_zones WHERE zone = 'p' AND grade = 3 AND size_max = 6"
                ).fetchone()
                self.assertEqual(decimal_row, ("shaft", 14.5, 12.0))

                skipped_empty_pairs = conn.execute(
                    "SELECT COUNT(*) FROM tolerance_zones WHERE source_file = 'table_26.csv' AND size_min = 6"
                ).fetchone()[0]
                self.assertEqual(skipped_empty_pairs, 0)

                metadata = dict(conn.execute("SELECT key, value FROM metadata").fetchall())
                self.assertEqual(metadata["imported_tables"], "1,2,3,4,22,23,26")
            finally:
                conn.close()

    def test_rejects_incomplete_tolerance_value_pair(self):
        with tempfile.TemporaryDirectory() as tmp_dir:
            csv_dir = Path(tmp_dir) / "src-tauri" / "data" / "csv"
            csv_dir.mkdir(parents=True)
            (csv_dir / "table_26.csv").write_text(INCOMPLETE_PAIR_CSV, encoding="utf-8")

            db_path = Path(tmp_dir) / "src-tauri" / "data" / "iso286.sqlite"

            with self.assertRaisesRegex(Exception, "both p3_es_um and p3_ei_um must be present"):
                import_database(csv_dir, db_path)

    def test_required_zone_gate_accepts_exact_required_classes(self):
        with tempfile.TemporaryDirectory() as tmp_dir:
            csv_dir = Path(tmp_dir) / "src-tauri" / "data" / "csv"
            csv_dir.mkdir(parents=True)
            (csv_dir / "required.csv").write_text(REQUIRED_ZONES_CSV, encoding="utf-8")

            db_path = Path(tmp_dir) / "src-tauri" / "data" / "iso286.sqlite"
            import_database(csv_dir, db_path, validate_required_zones=True)

            conn = sqlite3.connect(db_path)
            try:
                self.assertEqual(
                    conn.execute("SELECT COUNT(*) FROM tolerance_zones").fetchone()[0],
                    31,
                )
            finally:
                conn.close()

    def test_required_zone_gate_rejects_wrong_class_names(self):
        with tempfile.TemporaryDirectory() as tmp_dir:
            csv_dir = Path(tmp_dir) / "src-tauri" / "data" / "csv"
            csv_dir.mkdir(parents=True)
            (csv_dir / "required.csv").write_text(MISSING_REQUIRED_ZONES_CSV, encoding="utf-8")

            db_path = Path(tmp_dir) / "src-tauri" / "data" / "iso286.sqlite"

            with self.assertRaisesRegex(Exception, "missing hole zones: Y"):
                import_database(csv_dir, db_path, validate_required_zones=True)


if __name__ == "__main__":
    unittest.main()
