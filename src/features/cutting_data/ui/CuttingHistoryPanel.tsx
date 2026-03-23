import { cuttingDataFieldConfig } from "./cuttingDataFieldConfig";
import { formatNumber } from "@shared/ui/format/formatNumber";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { HistoryCard } from "@shared/ui/layout/container/HistoryCard/HistoryCard";
import { Button } from "@shared/ui/primitives/Button/Button";

import "./CuttingHistoryPanel.css";

type Props = {
  history: any[];
  onLoad(entry: any): void;
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
    <div className="cutting-history">

      <h3 className="cutting-history-title">
        Saved results
      </h3>

      {history.length === 0 && (
        <div className="cutting-history-empty">
          No saved results yet
        </div>
      )}

      {history.length > 0 && (
        <div className="cutting-history-list">
          {history.map((entry) => {
            const items = buildItems(entry, decimals);

            return (
              <HistoryCard
                key={entry.id}
                items={items}
                columns={2}
                onClick={() => onLoad(entry)}
                onDelete={() => onDelete(entry.id)}
              />
            );
          })}
        </div>
      )}

      {history.length > 0 && (
        <div className="cutting-history-actions">
  <Button
    variant="secondary"
    size="small"
    onClick={onClear}
  >
    Clear all results
  </Button>
</div>
      )}

    </div>
  );
}

/* =====================================================
   HELPERS
===================================================== */

function buildItems(entry: any, decimals: number) {
  return cuttingDataFieldConfig
    .map((config) => {
      const field = entry.form.fields[config.key];
      if (!field) return null;

      const rawValue = field.machineValue ?? field.value;
      if (rawValue == null) return null;

      const num =
        typeof rawValue === "number"
          ? rawValue
          : Number(rawValue);

      if (isNaN(num)) return null;

      return {
        label: config.shortLabel ?? config.label,
        value: formatNumber(num, decimals),
        unit: config.unit,
      };
    })
    .filter(Boolean) as {
      label: string;
      value: string;
      unit?: string;
    }[];
}