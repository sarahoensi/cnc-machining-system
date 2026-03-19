// features/cuttingData/ui/cuttingDataFieldConfig.ts

import type { CuttingDataKey } from "../domain/cuttingDataForm";
import { cuttingDataTooltips } from "./cuttingDataTooltips";

export type CuttingDataFieldConfig = {
  key: CuttingDataKey;
  label: string;
  tooltip?: string;
  unit?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export const cuttingDataFieldConfig: CuttingDataFieldConfig[] = [
  {
    key: "diameter",
    label: "Tool diameter D",
    tooltip: cuttingDataTooltips.diameter,
    unit: "mm",
    autoFocus: true,
  },
  {
    key: "teeth",
    label: "Toothcount z",
    tooltip: cuttingDataTooltips.teeth,
  },
  {
    key: "cutting_speed",
    label: "Cutting speed Vc",
    tooltip: cuttingDataTooltips.cutting_speed,
    unit: "m/min",
  },
  {
    key: "rpm",
    label: "Rotations n",
    tooltip: cuttingDataTooltips.rpm,
    unit: "rpm",
  },

  
  {
    key: "feed_rate",
    label: "Feed rate F",
    tooltip: cuttingDataTooltips.feed_rate,
    unit: "mm/min",
  },
  {
    key: "chip_load",
    label: "Chip load fz",
    tooltip: cuttingDataTooltips.chip_load,
    unit: "mm/tooth",
  },

  
];