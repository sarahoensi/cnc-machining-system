// src/features/cylinder_weight/domain/validateCylinderWeightForm.ts

import { FieldState } from "@shared/form";
import { CylinderWeightExtras, CylinderWeightKey } from "./cylinderWeightForm";

export function validateCylinderWeightForm(
  fields: Record<CylinderWeightKey, FieldState>,
  _extras: CylinderWeightExtras
): string[] | null {
  const errors: string[] = [];

  if (!fields.outer_diameter_mm.value) {
    errors.push("Outer diameter is required");
  }
  if (!fields.length_mm.value) {
    errors.push("Length is required");
  }

  return errors.length > 0 ? errors : null;
}

