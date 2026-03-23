// features/cuttingData/ui/CuttingHistoryPanel.tsx

import { cuttingDataFieldConfig } from "../cuttingDataFieldConfig";
import { formatNumber } from "@shared/ui/format/formatNumber";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { HistoryCard } from "@shared/ui/layout/container/HistoryCard/HistoryCard";

import "./CuttingHistoryPanel.css"

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
      <h3>Saved results</h3>

      {history.length === 0 && (
        <div className="cutting-history-empty">
          No saved results yet
        </div>
      )}

      {history.map((entry) => {
        const items = cuttingDataFieldConfig
          .map((config) => {
            const field = entry.form.fields[config.key];
            if (!field) return null;

            const rawValue =
              field.machineValue ?? field.value;

            if (rawValue == null) return null;

            const num =
              typeof rawValue === "number"
                ? rawValue
                : Number(rawValue);

            if (isNaN(num)) return null;

            return {
              label:
                config.shortLabel ?? config.label,
              value: formatNumber(num, decimals),
              unit: config.unit,
            };
          })
          .filter(Boolean) as {
          label: string;
          value: string;
          unit?: string;
        }[];

        return (
          <HistoryCard
            key={entry.id}
            items={items}
            columns={2} // ← enkelt å endre til 3 senere
            onClick={() => onLoad(entry)}
            onDelete={() => onDelete(entry.id)}
          />
        );
      })}

       {/* Clear all button */}
    {history.length > 0 && (
      <button
        className="cutting-history-clear"
        onClick={onClear}
      >
        Clear all results
      </button>
    )}
    </div>
  );
}