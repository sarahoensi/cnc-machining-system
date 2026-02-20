// features/right_triangle/triangleFieldConfig.ts

import type { TriangleKey } from "../domain/triangleForm";
import { triangleTooltips } from "./triangleTooltips";

export type TriangleFieldConfig = {
  key: TriangleKey;
  label: string;
  unit?: string;
  tooltip?: string;
  autoFocus?: boolean;
};

export const triangleFieldConfig: TriangleFieldConfig[] = [
  {
    key: "a",
    label: "Katet a",
    unit: "mm",
    tooltip: triangleTooltips.a,
    autoFocus: true,
  },
  {
    key: "b",
    label: "Katet b",
    unit: "mm",
    tooltip: triangleTooltips.b,
  },
  {
    key: "c",
    label: "Hypotenus c",
    unit: "mm",
    tooltip: triangleTooltips.c,
  },
  {
    key: "alpha",
    label: "Vinkel α",
    unit: "°",
    tooltip: triangleTooltips.alpha,
  },
  {
    key: "beta",
    label: "Vinkel β",
    unit: "°",
    tooltip: triangleTooltips.beta,
  },
];