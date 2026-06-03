import type { ToleranceFormInput } from "./buildRequest";

export function validateToleranceForm(input: ToleranceFormInput) {
  const errors: Partial<Record<keyof ToleranceFormInput, string>> = {};
  const nominal = Number(input.nominal.replace(",", "."));
  const needsHole = input.mode === "hole";
  const needsShaft = input.mode === "shaft";

  if (needsHole && !input.holeLetter.trim()) {
    errors.holeLetter = "Hole tolerance letter is required";
  }

  if (needsHole && !input.holeGrade.trim()) {
    errors.holeGrade = "Hole tolerance grade is required";
  }

  if (needsShaft && !input.shaftLetter.trim()) {
    errors.shaftLetter = "Shaft tolerance letter is required";
  }

  if (needsShaft && !input.shaftGrade.trim()) {
    errors.shaftGrade = "Shaft tolerance grade is required";
  }

  if (!input.nominal.trim()) {
    errors.nominal = "Nominal size is required";
  } else if (!Number.isFinite(nominal) || nominal <= 0) {
    errors.nominal = "Nominal size must be greater than zero";
  }

  return errors;
}
