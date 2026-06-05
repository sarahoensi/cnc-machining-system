// features/cuttingData/ui/history/CuttingHistoryPanel.tsx

import { cuttingDataFieldConfig } from "../cuttingDataFieldConfig";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { buildFieldHistoryItems } from "@shared/savedResults";
import type { SavedResultEntry } from "@shared/savedResults";
import { SavedResultsPanel } from "./SavedResultsPanel/SavedResultsPanel";
import type { createInitialCuttingDataForm } from "../../domain/cuttingDataForm";

type Props = {
  history: SavedResultEntry<ReturnType<typeof createInitialCuttingDataForm>>[];
  onLoad(entry: SavedResultEntry<ReturnType<typeof createInitialCuttingDataForm>>): void;
  onDelete(id: string): void;
  onClear(): void;
};

export function CuttingHistoryPanel({
  history,
  onLoad,
  onDelete,
  onClear,
}: Props) {
  const { decimals } = useDisplaySettings();

  return (
    <SavedResultsPanel
      entries={history}
      buildItems={(entry) =>
        buildFieldHistoryItems(entry.form.fields, cuttingDataFieldConfig, decimals)
      }
      onLoad={onLoad}
      onDelete={onDelete}
      onClear={onClear}
    />
  );
}

