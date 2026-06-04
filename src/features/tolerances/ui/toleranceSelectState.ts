import type { ToleranceFormState } from "../domain/toleranceForm";
import { gradesForZone } from "../domain/toleranceOptions";

export function getToleranceSelectState(form: ToleranceFormState) {
  const { options } = form.extras;

  const holeLetter = form.fields.hole_letter.value;
  const holeGrade = form.fields.hole_grade.value;
  const shaftLetter = form.fields.shaft_letter.value;
  const shaftGrade = form.fields.shaft_grade.value;

  const holeGrades = gradesForZone(options.holes, holeLetter);
  const shaftGrades = gradesForZone(options.shafts, shaftLetter);

  return {
    holeLetter,
    holeGrade,
    shaftLetter,
    shaftGrade,

    holeLetterOptions: toSelectOptions(options.holes.map((option) => option.zone)),
    holeGradeOptions: toSelectOptions(holeGrades),
    shaftLetterOptions: toSelectOptions(options.shafts.map((option) => option.zone)),
    shaftGradeOptions: toSelectOptions(shaftGrades),
  };
}

function toSelectOptions(values: string[]) {
  return values.map((value) => ({
    value,
    label: value,
  }));
}