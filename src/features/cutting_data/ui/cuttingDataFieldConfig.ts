// features/cuttingData/ui/cuttingDataFieldConfig.ts

import type { CuttingDataKey } from "../domain/cuttingDataForm";

export type CuttingDataFieldConfig = {
  key: CuttingDataKey;
  label: string;
  unit?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export const cuttingDataFieldConfig: CuttingDataFieldConfig[] = [
  {
    key: "diameter",
    label: "Tool diameter D",
    unit: "mm",
    autoFocus: true,
  },
  {
    key: "teeth",
    label: "Toothcount z",
    unit: "",
  },
  {
    key: "cutting_speed",
    label: "Cutting speed Vc",
    unit: "m/min",
  },
  {
    key: "rpm",
    label: "Rotations n",
    unit: "rpm",
  },

  
  {
    key: "feed_rate",
    label: "Feed rate F",
    unit: "mm/min",
  },
  {
    key: "chip_load",
    label: "Chip load fz",
    unit: "mm/tooth",
  },

  
];