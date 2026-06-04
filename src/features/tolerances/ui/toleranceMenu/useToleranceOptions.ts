// features/tolerances/ui/useToleranceOptions.ts

import { useMemo } from "react";

import type { ToleranceFormState } from "../../domain/toleranceForm";
import { gradesForZone } from "../../domain/toleranceOptions";

export function useToleranceOptions(form: ToleranceFormState) {
  const { options } = form.extras;

  const holeLetter = form.fields.hole_letter.value;
  const holeGrade = form.fields.hole_grade.value;
  const shaftLetter = form.fields.shaft_letter.value;
  const shaftGrade = form.fields.shaft_grade.value;

  const holeGrades = useMemo(
    () => gradesForZone(options.holes, holeLetter),
    [holeLetter, options.holes],
  );

  const shaftGrades = useMemo(
    () => gradesForZone(options.shafts, shaftLetter),
    [shaftLetter, options.shafts],
  );

  const holeLetterOptions = useMemo(
    () =>
      options.holes.map((option) => ({
        value: option.zone,
        label: option.zone,
      })),
    [options.holes],
  );

  const shaftLetterOptions = useMemo(
    () =>
      options.shafts.map((option) => ({
        value: option.zone,
        label: option.zone,
      })),
    [options.shafts],
  );

  const holeGradeOptions = useMemo(
    () =>
      holeGrades.map((value) => ({
        value,
        label: value,
      })),
    [holeGrades],
  );

  const shaftGradeOptions = useMemo(
    () =>
      shaftGrades.map((value) => ({
        value,
        label: value,
      })),
    [shaftGrades],
  );

  return {
    holeLetter,
    holeGrade,
    shaftLetter,
    shaftGrade,

    holeGrades,
    shaftGrades,

    holeLetterOptions,
    holeGradeOptions,
    shaftLetterOptions,
    shaftGradeOptions,
  };
}