import type { FormState } from "@shared/form/types/forms";
import { emptyField, userField } from "@shared/form/types/fields";

import type { ToleranceMode, ToleranceOptionsResponse } from "../api/types";
import type { ToleranceFormInput } from "./buildRequest";

export type ToleranceKey =
  | "nominal"
  | "hole_letter"
  | "hole_grade"
  | "shaft_letter"
  | "shaft_grade"
  | "upper_um"
  | "lower_um"
  | "min_mm"
  | "max_mm";

export type ToleranceExtras = {
  mode: ToleranceMode;
  options: ToleranceOptionsResponse;
  loadingOptions: boolean;
};

export type ToleranceFormState = FormState<ToleranceKey, ToleranceExtras>;

const resultKeys = ["upper_um", "lower_um", "min_mm", "max_mm"] as const;

export function createInitialToleranceForm(): ToleranceFormState {
  return {
    status: "editing",
    fields: {
      nominal: emptyField(),
      hole_letter: userField("H"),
      hole_grade: userField("7"),
      shaft_letter: userField("g"),
      shaft_grade: userField("6"),
      upper_um: resultField(),
      lower_um: resultField(),
      min_mm: resultField(),
      max_mm: resultField(),
    },
    extras: {
      mode: "hole",
      options: {
        holes: [],
        shafts: [],
      },
      loadingOptions: true,
    },
  };
}

export function migrateToleranceForm(
  form: ToleranceFormState,
): ToleranceFormState {
  const initial = createInitialToleranceForm();
  const legacyExtras = form.extras as ToleranceExtras & {
    holeLetter?: string;
    holeGrade?: string;
    shaftLetter?: string;
    shaftGrade?: string;
  };
  const needsMigration =
    !form.fields.hole_letter ||
    !form.fields.hole_grade ||
    !form.fields.shaft_letter ||
    !form.fields.shaft_grade ||
    !form.fields.upper_um ||
    !form.fields.lower_um ||
    !form.fields.min_mm ||
    !form.fields.max_mm ||
    !form.extras.options ||
    form.extras.loadingOptions == null ||
    legacyExtras.holeLetter != null ||
    legacyExtras.holeGrade != null ||
    legacyExtras.shaftLetter != null ||
    legacyExtras.shaftGrade != null;

  if (!needsMigration) return normalizeSolvedResultFields(form);

  return normalizeSolvedResultFields({
    ...initial,
    ...form,
    fields: {
      ...initial.fields,
      ...form.fields,
      hole_letter:
        form.fields.hole_letter ??
        userField(legacyExtras.holeLetter ?? initial.fields.hole_letter.value),
      hole_grade:
        form.fields.hole_grade ??
        userField(legacyExtras.holeGrade ?? initial.fields.hole_grade.value),
      shaft_letter:
        form.fields.shaft_letter ??
        userField(
          legacyExtras.shaftLetter ?? initial.fields.shaft_letter.value,
        ),
      shaft_grade:
        form.fields.shaft_grade ??
        userField(legacyExtras.shaftGrade ?? initial.fields.shaft_grade.value),
      upper_um: {
        ...initial.fields.upper_um,
        ...form.fields.upper_um,
      },
      lower_um: {
        ...initial.fields.lower_um,
        ...form.fields.lower_um,
      },
      min_mm: {
        ...initial.fields.min_mm,
        ...form.fields.min_mm,
      },
      max_mm: {
        ...initial.fields.max_mm,
        ...form.fields.max_mm,
      },
    },
    extras: {
      ...initial.extras,
      mode: form.extras.mode ?? initial.extras.mode,
      options: form.extras.options ?? initial.extras.options,
      loadingOptions:
        form.extras.loadingOptions ?? initial.extras.loadingOptions,
    },
  });
}

export function buildToleranceFormInput(
  form: ToleranceFormState,
): ToleranceFormInput {
  return {
    mode: form.extras.mode,
    nominal: form.fields.nominal.value,
    holeLetter: form.fields.hole_letter.value,
    holeGrade: form.fields.hole_grade.value,
    shaftLetter: form.fields.shaft_letter.value,
    shaftGrade: form.fields.shaft_grade.value,
  };
}

export function resultField() {
  return emptyField({ locked: true });
}

function normalizeSolvedResultFields(
  form: ToleranceFormState,
): ToleranceFormState {
  if (form.status !== "solved") return form;

  let changed = false;
  const fields = { ...form.fields };

  for (const key of resultKeys) {
    const field = fields[key];
    if (field.source !== "machine" || !field.locked) continue;

    fields[key] = {
      ...field,
      locked: false,
    };
    changed = true;
  }

  if (!changed) return form;

  return {
    ...form,
    fields,
  };
}
