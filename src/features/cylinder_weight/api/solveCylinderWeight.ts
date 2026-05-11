import { buildSolveCylinderWeightRequest } from "../domain/buildSolveRequest";
import {
  CylinderWeightExtras,
  CylinderWeightKey,
} from "../domain/cylinderWeightForm";
import { solveCylinderWeightApi } from "./client";

export async function solveCylinderWeight(
  input: Partial<Record<CylinderWeightKey, number>>,
  extras: CylinderWeightExtras
): Promise<Partial<Record<CylinderWeightKey, number>>> {
  const request = buildSolveCylinderWeightRequest(input, extras);
  const result = await solveCylinderWeightApi(request);

  return {
    outer_diameter_mm: result.outer_diameter_mm,
    inner_diameter_mm: result.inner_diameter_mm,
    length_mm: result.length_mm,
    mass_kg: result.mass_kg,
  };
}
