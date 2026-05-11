import { InfoDialog } from "@shared/ui/components/overlay/InfoDialog/InfoDialog";
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
          </>
        ) : null}
      </InfoDialog>

      <InfoDialog
        open={!!exportSummary}
        title="Export completed"
        onClose={onCloseExport}
      >
        {exportSummary ? (
          <p>Exported {exportSummary.exported} materials successfully.</p>
        ) : null}
      </InfoDialog>
    </>
  );
}