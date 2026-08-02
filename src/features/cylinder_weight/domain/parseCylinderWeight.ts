// src/features/cylinder_weight/domain/parseCylinderWeight.ts

import { safeParseDecimal } from "@shared/parsing/decimalParser";
import { FieldState } from "@shared/form/types";
import { CylinderWeightExtras, CylinderWeightKey } from "./cylinderWeightForm";

export function parseCylinderWeight(
  fields: Record<CylinderWeightKey, FieldState>,
  _extras: CylinderWeightExtras,
): Partial<Record<CylinderWeightKey, number>> | null {
  const parsed: Partial<Record<CylinderWeightKey, number>> = {};
  const keys: CylinderWeightKey[] = [
    "outer_diameter_mm",
    "inner_diameter_mm",
    "length_mm",
  ];

  for (const key of keys) {
    const value = fields[key].value;

    if (!value) continue;

    const normalized = safeParseDecimal(value);
    if (normalized == null) return null;

    parsed[key] = normalized;
  }

  return parsed;
}
