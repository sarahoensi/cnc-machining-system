// src/features/cylinder_weight/ui/materials/feedback/MaterialResultDialogs.tsx

import { InfoDialog } from "@shared/ui/components/overlay/InfoDialog/InfoDialog";
import { Button } from "@shared/ui/primitives/Button/Button";
import { useEffect, useState } from "react";
import { ExportSummary, ImportSummary } from "../types";

type Props = {
  importSummary: ImportSummary | null;
  exportSummary: ExportSummary | null;
  onCloseImport: () => void;
  onCloseExport: () => void;
};

export function MaterialResultDialogs({
  importSummary,
  exportSummary,
  onCloseImport,
  onCloseExport,
}: Props) {
  const [showImportDetails, setShowImportDetails] = useState(false);
  const [showExportDetails, setShowExportDetails] = useState(false);

  useEffect(() => {
    if (!importSummary) {
      setShowImportDetails(false);
    }
  }, [importSummary]);

  useEffect(() => {
    if (!exportSummary) {
      setShowExportDetails(false);
    }
  }, [exportSummary]);

  const duplicateRows = importSummary?.skipped.filter((row) => row.reason === "duplicate") ?? [];
  const invalidRows = importSummary?.skipped.filter((row) => row.reason === "invalid") ?? [];

  return (
    <>
      <InfoDialog
        open={!!importSummary}
        title="Import completed"
        onClose={onCloseImport}
      >
        {importSummary ? (
          <>
            <p>Imported {importSummary.imported} materials.</p>
            <p>Skipped {importSummary.skippedDuplicates} duplicates.</p>
            <p>Skipped {importSummary.skippedInvalid} invalid materials.</p>
            <Button variant="link" onClick={() => setShowImportDetails((v) => !v)}>
              {showImportDetails ? "View less" : "View more"}
            </Button>

            {showImportDetails ? (
              <>
                {importSummary.added.length > 0 ? (
                  <>
                    <p><strong>Added</strong></p>
                    <ul>
                      {importSummary.added.map((row, index) => (
                        <li key={`${row.name}-${row.density_kg_m3}-${index}`}>
                          {row.name} - {row.density_kg_m3} kg/m3
                          {row.original_name ? ` (from ${row.original_name})` : ""}
                        </li>
                      ))}
                    </ul>
                  </>
                ) : null}

                {duplicateRows.length > 0 ? (
                  <>
                    <p><strong>Skipped duplicates</strong></p>
                    <ul>
                      {duplicateRows.map((row, index) => (
                        <li key={`dup-${row.name ?? "unknown"}-${index}`}>
                          {(row.name ?? "Unknown material")}{" "}
                          {row.density_kg_m3 != null ? `- ${row.density_kg_m3} kg/m3` : ""}
                          {`: ${row.message}`}
                        </li>
                      ))}
                    </ul>
                  </>
                ) : null}

                {invalidRows.length > 0 ? (
                  <>
                    <p><strong>Skipped invalid</strong></p>
                    <ul>
                      {invalidRows.map((row, index) => (
                        <li key={`inv-${row.name ?? "unknown"}-${index}`}>
                          {(row.name ?? "Unknown material")}{" "}
                          {row.density_kg_m3 != null ? `- ${row.density_kg_m3} kg/m3` : ""}
                          {`: ${row.message}`}
                        </li>
                      ))}
                    </ul>
                  </>
                ) : null}
              </>
            ) : null}
          </>
        ) : null}
      </InfoDialog>

      <InfoDialog
        open={!!exportSummary}
        title="Export completed"
        onClose={onCloseExport}
      >
        {exportSummary ? (
          <>
            <p>Exported {exportSummary.exported} materials successfully.</p>
            <Button variant="link" onClick={() => setShowExportDetails((v) => !v)}>
              {showExportDetails ? "View less" : "View more"}
            </Button>

            {showExportDetails ? (
              <>
                <p><strong>Exported materials</strong></p>
                <ul>
                  {exportSummary.materials.map((row, index) => (
                    <li key={`${row.name}-${row.density_kg_m3}-${index}`}>
                      {row.name} - {row.density_kg_m3} kg/m3
                    </li>
                  ))}
                </ul>
              </>
            ) : null}
          </>
        ) : null}
      </InfoDialog>
    </>
  );
}

