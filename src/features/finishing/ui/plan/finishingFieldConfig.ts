// features/finishing/ui/plan/finishingFieldConfig.ts

import type { FinishingKey } from "../../domain/plan/finishingForm";

export type FinishingFieldConfig = {
  key: FinishingKey;
  label: string;
  unit?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export const finishingFieldConfig: FinishingFieldConfig[] = [
  {
    key: "start_diameter",
    label: "Start diameter",
    unit: "mm",
    autoFocus: true,
  },
  {
    key: "target_diameter",
    label: "Target diameter",
    unit: "mm",
  },
  {
    key: "cuts",
    label: "Number of cuts",
  },
  {
    key: "radial_engagement",
    label: "Radial engagement",
    unit: "mm",
  },
];