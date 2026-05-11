import { CylinderWeightKey } from "../domain/cylinderWeightForm";

type CylinderWeightFieldConfig = {
  key: CylinderWeightKey;
  label: string;
  unit?: string;
  readOnly?: boolean;
  autoFocus?: boolean;
};

export const cylinderWeightFieldConfig: CylinderWeightFieldConfig[] = [
  {
    key: "outer_diameter_mm",
    label: "Outer diameter",
    unit: "mm",
    autoFocus: true,
  },
  {
    key: "inner_diameter_mm",
    label: "Inner diameter",
    unit: "mm",
  },
  {
    key: "length_mm",
    label: "Length",
    unit: "mm",
  },
  {
    key: "mass_kg",
    label: "Mass",
    unit: "kg",
    readOnly: true,
  },
];
