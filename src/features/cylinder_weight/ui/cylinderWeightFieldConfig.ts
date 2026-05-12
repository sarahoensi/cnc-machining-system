// src/features/cylinder_weight/ui/cylinderWeightFieldConfig.ts

import { CylinderWeightKey } from "../domain/cylinderWeightForm";

type CylinderWeightFieldConfig = {
  key: CylinderWeightKey;
  label: string;
  tooltip?: string;
  unit?: string;
  readOnly?: boolean;
  autoFocus?: boolean;
};

export const cylinderWeightFieldConfig: CylinderWeightFieldConfig[] = [
  {
    key: "outer_diameter_mm",
    label: "Outer diameter",
    tooltip: "Outside diameter of the cylinder.",
    unit: "mm",
    autoFocus: true,
  },
  {
    key: "inner_diameter_mm",
    label: "Inner diameter",
    tooltip: "Leave blank for a solid cylinder.",
    unit: "mm",
  },
  {
    key: "length_mm",
    label: "Length",
    tooltip: "Total cylinder length along its axis.",
    unit: "mm",
  },
  {
    key: "mass_kg",
    label: "Mass",
    tooltip: "Calculated mass from geometry and selected material density.",
    unit: "kg",
    readOnly: true,
  },
];

