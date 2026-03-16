// features/finishing/ui/plan/finishingFieldConfig.ts

import { FinishingKey } from "../domain/finishingForm";

export type FinishingFieldConfig = {
  key: FinishingKey;
  label: string;
  unit?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export const finishingFieldConfig: FinishingFieldConfig[] = [
  {
    key: "start_diameter_mm",
    label: "Start diameter",
    unit: "mm",
    autoFocus: true,
  },
  {
    key: "target_diameter_mm",
    label: "Target diameter",
    unit: "mm",
  },
  {
    key: "cuts",
    label: "Number of cuts",
  },
  {
    key: "radial_engagement_mm",
    label: "Radial engagement",
    unit: "mm",
  },
];