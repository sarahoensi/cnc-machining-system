// features/tolerances/domain/parseTolerance.ts

import { safeParseDecimal } from "@shared/parsing/decimalParser";
import type { FieldState } from "@shared/form/types";

import type { ToleranceObjectType } from "../api/types";
import type { ToleranceExtras, ToleranceKey } from "./toleranceForm";

export type ParsedToleranceInput = {
  feature: ToleranceObjectType;
  nominalMm: number;
  code: string;
};

export function parseTolerance(
  fields: Record<ToleranceKey, FieldState>,
  extras: ToleranceExtras,
): ParsedToleranceInput | null {
  const nominalMm = safeParseDecimal(fields.nominal.value);
  if (nominalMm == null) return null;

  const letter =
    extras.mode === "hole" ? fields.hole_letter.value : fields.shaft_letter.value;
  const grade =
    extras.mode === "hole" ? fields.hole_grade.value : fields.shaft_grade.value;

  return {
    feature: extras.mode,
    nominalMm,
    code: `${letter}${grade}`.trim(),
  };
}
