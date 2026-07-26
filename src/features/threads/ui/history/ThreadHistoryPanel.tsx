import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { buildFieldHistoryItems } from "@shared/savedResults";
import type { SavedResultEntry } from "@shared/savedResults";

import type { ThreadFormState } from "../../domain/threadForm";
import { threadHistoryFieldConfig } from "../threadFieldConfig";
import { SavedResultsPanel } from "@features/cuttingData/ui/history/SavedResultsPanel/SavedResultsPanel";

type Props = {
  history: SavedResultEntry<ThreadFormState>[];
  onLoad(entry: SavedResultEntry<ThreadFormState>): void;
  onDelete(id: string): void;
  onClear(): void;
};

export function ThreadHistoryPanel({
  history,
  onLoad,
  onDelete,
  onClear,
}: Props) {
  const { decimals } = useDisplaySettings();

  return (
    <SavedResultsPanel
      title="Saved threads"
      emptyText="No saved threads yet"
      clearText="Clear all threads"
      entries={history}
      columns={2}
      buildItems={(entry) => [
        {
          label: "Thread",
          value: `${entry.form.fields.size.value} ${formatPitch(entry.form.fields.pitch.value, entry.form.extras.type)}`,
        },
        ...buildFieldHistoryItems(
          entry.form.fields,
          threadHistoryFieldConfig,
          decimals,
        ),
      ]}
      onLoad={onLoad}
      onDelete={onDelete}
      onClear={onClear}
    />
  );
}

function formatPitch(value: string, type: ThreadFormState["extras"]["type"]) {
  if (!value) return "";
  if (type === "unified") {
    const [series, pitch] = value.split(":", 2);
    const suffix = series ? ` (${series.toUpperCase()})` : "";

    return pitch ? `- ${pitch} TPI${suffix}` : "";
  }

  return type === "metric" ? `x ${value}` : `- ${value} TPI`;
}
