// features/tolerances/api/solveTolerance.ts

import type { ToleranceKey } from "../domain/toleranceForm";
import type { ParsedToleranceInput } from "../domain/parseTolerance";
import { lookupIso286ToleranceApi } from "./client";

export async function solveTolerance(
  input: ParsedToleranceInput,
): Promise<Partial<Record<ToleranceKey, number>>> {
  const result = await lookupIso286ToleranceApi({
    feature: input.feature,
    nominalMm: input.nominalMm,
    code: input.code,
  });

  return {
    upper_um: result.upper_um / 1000,
    lower_um: result.lower_um / 1000,
    min_mm: result.min_mm,
    max_mm: result.max_mm,
  };
}
