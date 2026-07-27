import { clearMachineFields, handleUserEdit } from "@shared/form/engine/formEngine";

import { userField } from "@shared/form/types/fields";

import type { ToleranceObjectType, ToleranceOptionsResponse } from "../api/types";

import { gradesForZone } from "./toleranceOptions";

import type { ToleranceFormState, ToleranceKey } from "./toleranceForm";

const validInputSets: readonly (readonly ToleranceKey[])[] = [
  ["nominal", "hole_letter", "hole_grade", "shaft_letter", "shaft_grade"],
];

const mutuallyExclusivePairs: readonly (readonly [ToleranceKey, ToleranceKey])[] = [];

export function applyToleranceUserEdit(
  form: ToleranceFormState,
  key: ToleranceKey,
  value: string,
) {
  return handleUserEdit(form, key, value, validInputSets, mutuallyExclusivePairs);
}

export function applyToleranceLetterChange(
  form: ToleranceFormState,
  options: ToleranceOptionsResponse,
  feature: ToleranceObjectType,
  value: string,
) {
  const isHole = feature === "hole";

  const nextGrades = isHole
    ? gradesForZone(options.holes, value)
    : gradesForZone(options.shafts, value);

  const next = applyToleranceUserEdit(
    form,
    isHole ? "hole_letter" : "shaft_letter",
    value,
  );

  const currentGrade = isHole
    ? form.fields.hole_grade.value
    : form.fields.shaft_grade.value;
  const nextGrade = pickGradeForClassChange(nextGrades, currentGrade);

  return patchSelectionFields(next, {
    [isHole ? "hole_grade" : "shaft_grade"]: nextGrade,
  });
}

export function applyToleranceGradeChange(
  form: ToleranceFormState,
  feature: ToleranceObjectType,
  value: string,
) {
  return applyToleranceUserEdit(
    form,
    feature === "hole" ? "hole_grade" : "shaft_grade",
    value,
  );
}

export function patchSelectionFields(
  form: ToleranceFormState,
  patch: Partial<Record<ToleranceKey, string>>,
): ToleranceFormState {
  const fields = clearMachineFields(form.fields);

  for (const key in patch) {
    const typedKey = key as ToleranceKey;

    fields[typedKey] = userField(patch[typedKey] ?? "");
  }

  return {
    ...form,
    status: "editing",
    fields,
    formError: undefined,
  };
}

function pickGradeForClassChange(availableGrades: string[], currentGrade: string) {
  if (availableGrades.includes(currentGrade)) return currentGrade;
  if (availableGrades.includes("7")) return "7";

  return availableGrades[0] ?? "";
}
