// src/features/cylinder_weight/domain/buildRequest.ts

import { SolveCylinderWeightRequest } from "../api/types";
import { CylinderWeightExtras, CylinderWeightKey } from "./cylinderWeightForm";

export function buildSolveCylinderWeightRequest(
  input: Partial<Record<CylinderWeightKey, number>>,
  extras: CylinderWeightExtras
): SolveCylinderWeightRequest {
  return {
    material_id: extras.materialId || undefined,
    outer_diameter_mm: input.outer_diameter_mm,
    inner_diameter_mm: input.inner_diameter_mm,
    length_mm: input.length_mm,
  };
}

