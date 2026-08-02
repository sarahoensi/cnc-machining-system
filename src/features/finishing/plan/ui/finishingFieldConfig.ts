// features/finishing/ui/plan/finishingFieldConfig.ts

import { FinishingKey } from "../domain/finishingForm";
import { finishingTooltips } from "./finishingPlanTooltip";

export type FinishingFieldConfig = {
  key: FinishingKey;
  label: string;
  unit?: string;
  tooltip?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export const finishingFieldConfig: FinishingFieldConfig[] = [
  {
    key: "start_diameter_mm",
    label: "Start diameter",
    unit: "mm",
    tooltip: finishingTooltips.start_diameter_mm,
    autoFocus: true,
  },
  {
    key: "target_diameter_mm",
    label: "Target diameter",
    unit: "mm",
    tooltip: finishingTooltips.target_diameter_mm,
  },
  {
    key: "cuts",
    label: "Number of cuts",
    tooltip: finishingTooltips.cuts,
  },
  {
    key: "radial_engagement_mm",
    label: "Radial engagement",
    unit: "mm",
    tooltip: finishingTooltips.radial_engagement_mm,
  },
];
