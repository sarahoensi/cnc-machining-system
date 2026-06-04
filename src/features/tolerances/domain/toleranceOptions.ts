// features/tolerances/domain/toleranceOptions.ts

import { userField } from "@shared/form/types/fields";

import type {
  ToleranceMode,
  ToleranceOption,
} from "../api/types";
import type {
  ToleranceFormState,
  ToleranceKey,
} from "./toleranceForm";

export function gradesForZone(options: ToleranceOption[], zone: string) {
  return (
    options.find((option) => option.zone === zone)?.grades.map(String) ?? []
  );
}

export function reconcileSelectionFields(
  fields: ToleranceFormState["fields"],
  options: ToleranceFormState["extras"]["options"],
): ToleranceFormState["fields"] {
  const hole = validSelection(
    options.holes,
    fields.hole_letter.value,
    fields.hole_grade.value,
    "H",
    "7",
  );

  const shaft = validSelection(
    options.shafts,
    fields.shaft_letter.value,
    fields.shaft_grade.value,
    "g",
    "6",
  );

  return {
    ...fields,
    hole_letter: userField(hole.zone),
    hole_grade: userField(hole.grade),
    shaft_letter: userField(shaft.zone),
    shaft_grade: userField(shaft.grade),
  };
}

export function preserveEquivalentToleranceSelection(
  form: ToleranceFormState,
  previousMode: ToleranceMode,
): ToleranceFormState["fields"] {
  if (previousMode === form.extras.mode) return form.fields;

  const sourcePrefix = previousMode;
  const targetPrefix = form.extras.mode;

  const sourceLetterKey = `${sourcePrefix}_letter` as ToleranceKey;
  const sourceGradeKey = `${sourcePrefix}_grade` as ToleranceKey;
  const targetLetterKey = `${targetPrefix}_letter` as ToleranceKey;
  const targetGradeKey = `${targetPrefix}_grade` as ToleranceKey;

  const targetOptions =
    targetPrefix === "hole"
      ? form.extras.options.holes
      : form.extras.options.shafts;

  const sourceLetter = form.fields[sourceLetterKey].value;
  const sourceGrade = form.fields[sourceGradeKey].value;

  const equivalentLetter =
    targetPrefix === "hole"
      ? sourceLetter.toUpperCase()
      : sourceLetter.toLowerCase();

  const targetOption = targetOptions.find(
    (option) => option.zone === equivalentLetter,
  );

  if (!targetOption) return form.fields;

  const nextGrade = targetOption.grades.includes(Number(sourceGrade))
    ? sourceGrade
    : form.fields[targetGradeKey].value;

  return {
    ...form.fields,
    [targetLetterKey]: userField(equivalentLetter),
    [targetGradeKey]: userField(nextGrade),
  };
}

function validSelection(
  options: ToleranceOption[],
  currentZone: string,
  currentGrade: string,
  preferredZone: string,
  preferredGrade: string,
) {
  const current = options.find((row) => row.zone === currentZone);

  if (current?.grades.includes(Number(currentGrade))) {
    return { zone: currentZone, grade: currentGrade };
  }

  const preferred = options.find((row) => row.zone === preferredZone);

  if (preferred) {
    return {
      zone: preferred.zone,
      grade: preferred.grades.includes(Number(preferredGrade))
        ? preferredGrade
        : String(preferred.grades[0] ?? ""),
    };
  }

  const fallback = options[0];

  return {
    zone: fallback?.zone ?? currentZone,
    grade:
      fallback?.grades[0] != null
        ? String(fallback.grades[0])
        : currentGrade,
  };
}