import { userField } from "@shared/form/types/fields";

import type {
  ToleranceMode,
  ToleranceOption,
} from "../api/types";

import type { ToleranceFormState } from "./toleranceForm";

export function gradesForZone(
  options: ToleranceOption[],
  zone: string,
) {
  return options.find((option) => option.zone === zone)?.grades.map(String) ?? [];
}

export function reconcileSelectionFields(
  fields: ToleranceFormState["fields"],
  options: ToleranceFormState["extras"]["options"],
): ToleranceFormState["fields"] {
  const hole = getValidSelection({
    options: options.holes,
    currentZone: fields.hole_letter.value,
    currentGrade: fields.hole_grade.value,
    preferredZone: "H",
    preferredGrade: "7",
  });

  const shaft = getValidSelection({
    options: options.shafts,
    currentZone: fields.shaft_letter.value,
    currentGrade: fields.shaft_grade.value,
    preferredZone: "h",
    preferredGrade: "7",
  });

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
  const nextMode = form.extras.mode;

  if (previousMode === nextMode) return form.fields;

  const source =
    previousMode === "hole"
      ? {
          letter: form.fields.hole_letter.value,
          grade: form.fields.hole_grade.value,
        }
      : {
          letter: form.fields.shaft_letter.value,
          grade: form.fields.shaft_grade.value,
        };

  const target =
    nextMode === "hole"
      ? {
          letterKey: "hole_letter" as const,
          gradeKey: "hole_grade" as const,
          options: form.extras.options.holes,
          equivalentLetter: source.letter.toUpperCase(),
        }
      : {
          letterKey: "shaft_letter" as const,
          gradeKey: "shaft_grade" as const,
          options: form.extras.options.shafts,
          equivalentLetter: source.letter.toLowerCase(),
        };

  const targetOption = target.options.find(
    (option) => option.zone === target.equivalentLetter,
  );

  if (!targetOption) return form.fields;

  const nextGrade = targetOption.grades.includes(Number(source.grade))
    ? source.grade
    : form.fields[target.gradeKey].value;

  return {
    ...form.fields,
    [target.letterKey]: userField(target.equivalentLetter),
    [target.gradeKey]: userField(nextGrade),
  };
}

type GetValidSelectionInput = {
  options: ToleranceOption[];
  currentZone: string;
  currentGrade: string;
  preferredZone: string;
  preferredGrade: string;
};

function getValidSelection({
  options,
  currentZone,
  currentGrade,
  preferredZone,
  preferredGrade,
}: GetValidSelectionInput) {
  const current = options.find((option) => option.zone === currentZone);

  if (current?.grades.includes(Number(currentGrade))) {
    return {
      zone: currentZone,
      grade: currentGrade,
    };
  }

  const preferred = options.find((option) => option.zone === preferredZone);

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