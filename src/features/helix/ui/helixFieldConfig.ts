// features/helix/ui/helixFieldConfig.ts

import type { HelixKey } from "../domain/helixForm";

export type HelixFieldConfig = {
  key: HelixKey;
  label: string;
  unit?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export const helixFieldConfig: HelixFieldConfig[] = [
  {
    key: "diameter",
    label: "Nominal diameter",
    unit: "mm",
    autoFocus: true,
  },
  {
    key: "tool_diameter",
    label: "Tool diameter",
    unit: "mm",
  },
  {
    key: "pitch",
    label: "Pitch",
    unit: "mm/rev",
  },
  {
    key: "angle",
    label: "Helix angle",
    unit: "°",
  },

  
];