// features/helix/ui/helixFieldConfig.ts

import type { HelixKey } from "../domain/helixForm";
import { helixTooltips } from "./helixTooltip";

export type HelixFieldConfig = {
  key: HelixKey;
  label: string;
  unit?: string;
  tooltip?: string;
  autoFocus?: boolean;
  readOnly?: boolean;
};

export const helixFieldConfig: HelixFieldConfig[] = [
  {
    key: "diameter",
    label: "Nominal diameter",
    unit: "mm",
    tooltip: helixTooltips.diameter,
    autoFocus: true,
  },
  {
    key: "tool_diameter",
    label: "Tool diameter",
    unit: "mm",
    tooltip: helixTooltips.tool_diameter,
  },
  {
    key: "pitch",
    label: "Pitch",
    unit: "mm/rev",
    tooltip: helixTooltips.pitch,
  },
  {
    key: "angle",
    label: "Helix angle",
    unit: "°",
    tooltip: helixTooltips.angle,
  },
];
