// features/tolerances/domain/validateToleranceForm.ts

import type { FieldState } from "@shared/form";

import type { ToleranceExtras, ToleranceKey } from "./toleranceForm";

export function validateToleranceForm(
  fields: Record<ToleranceKey, FieldState>,
  extras: ToleranceExtras,
) {
  const errors: string[] = [];
  const nominal = Number(fields.nominal.value.replace(",", "."));
  const needsHole = extras.mode === "hole";
  const needsShaft = extras.mode === "shaft";

  if (needsHole && !fields.hole_letter.value.trim()) {
    errors.push("Hole tolerance letter is required");
  }

  if (needsHole && !fields.hole_grade.value.trim()) {
    errors.push("Hole tolerance grade is required");
  }

  if (needsShaft && !fields.shaft_letter.value.trim()) {
    errors.push("Shaft tolerance letter is required");
  }

  if (needsShaft && !fields.shaft_grade.value.trim()) {
    errors.push("Shaft tolerance grade is required");
  }

  if (!fields.nominal.value.trim()) {
    errors.push("Nominal size is required");
  } else if (!Number.isFinite(nominal) || nominal <= 0) {
    errors.push("Nominal size must be greater than zero");
  }

  return errors.length > 0 ? errors : null;
}
