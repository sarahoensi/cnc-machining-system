// src/features/cylinder_weight/domain/cylinderWeightForm.ts

import type { FormState } from "@shared/form/types/forms";
import { emptyField, resultField } from "@shared/form/types/fields";

export type CylinderWeightKey =
  | "outer_diameter_mm"
  | "inner_diameter_mm"
  | "length_mm"
  | "mass_kg";

export type CylinderWeightExtras = {
  materialId: string;
  materialName?: string;
  densityKgM3?: number;
};

export function createInitialCylinderWeightForm(): FormState<
  CylinderWeightKey,
  CylinderWeightExtras
> {
  return {
    status: "editing",
    fields: {
      outer_diameter_mm: emptyField(),
      inner_diameter_mm: emptyField(),
      length_mm: emptyField(),
      mass_kg: resultField(),
    },
    extras: {
      materialId: "",
      materialName: undefined,
      densityKgM3: undefined,
    },
  };
}

