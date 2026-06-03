import type { ToleranceKey } from "../domain/toleranceForm";
import { toleranceTooltips } from "./toleranceTooltips";

export type ToleranceFieldConfig<K extends ToleranceKey = ToleranceKey> = {
  key: K;
  label: string;
  unit?: string;
  tooltip?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export type ToleranceInputKey = Extract<ToleranceKey, "nominal">;

export const toleranceInputFieldConfig: ToleranceFieldConfig<ToleranceInputKey>[] = [
  {
    key: "nominal",
    label: "Nominal size",
    unit: "mm",
    tooltip: toleranceTooltips.nominal,
    autoFocus: true,
  },
];

export const toleranceResultFieldConfig: ToleranceFieldConfig[] = [
  {
    key: "upper_um",
    label: "Upper",
    unit: "um",
    tooltip: toleranceTooltips.upper_um,
    readOnly: true,
  },
  {
    key: "lower_um",
    label: "Lower",
    unit: "um",
    tooltip: toleranceTooltips.lower_um,
    readOnly: true,
  },
  {
    key: "min_mm",
    label: "Minimum",
    unit: "mm",
    tooltip: toleranceTooltips.min_mm,
    readOnly: true,
  },
  {
    key: "max_mm",
    label: "Maximum",
    unit: "mm",
    tooltip: toleranceTooltips.max_mm,
    readOnly: true,
  },
];

export const toleranceModeConfig = {
  label: "Mode",
  tooltip: toleranceTooltips.mode,
};

export const toleranceClassFieldConfig = {
  classTooltip: toleranceTooltips.class,
  gradeTooltip: toleranceTooltips.grade,
};
