import type { FieldState } from "@shared/form/types/fields";
import { formatNumber } from "@shared/ui/format/formatNumber";

export type HistoryFieldConfig<K extends string> = {
  key: K;
  label: string;
  shortLabel?: string;
  unit?: string;
};

export type HistoryItem = {
  label: string;
  value: string;
  unit?: string;
};

export function buildFieldHistoryItems<K extends string>(
  fields: Record<K, FieldState>,
  configs: readonly HistoryFieldConfig<K>[],
  decimals: number,
): HistoryItem[] {
  return configs.map((config) => {
    const field = fields[config.key];
    const label = config.shortLabel ?? config.label;

    if (!field) {
      return {
        label,
        value: "-",
        unit: config.unit,
      };
    }

    const rawValue = field.machineValue ?? field.value;
    if (rawValue == null || rawValue === "") {
      return {
        label,
        value: "-",
        unit: config.unit,
      };
    }

    const numberValue =
      typeof rawValue === "number" ? rawValue : Number(rawValue);

    if (Number.isNaN(numberValue)) {
      return {
        label,
        value: "-",
        unit: config.unit,
      };
    }

    return {
      label,
      value: formatNumber(numberValue, decimals),
      unit: config.unit,
    };
  });
}
