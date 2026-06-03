import type { FormState } from "@shared/form/types/forms";
import { emptyField } from "@shared/form/types/fields";

import type { ToleranceMode, ToleranceOptionsResponse } from "../api/types";
import type { ToleranceFormInput } from "./buildRequest";

export type ToleranceKey =
  | "nominal"
  | "upper_um"
  | "lower_um"
  | "min_mm"
  | "max_mm";

export type ToleranceExtras = {
  mode: ToleranceMode;
  holeLetter: string;
  holeGrade: string;
  shaftLetter: string;
  shaftGrade: string;
  options: ToleranceOptionsResponse;
  loadingOptions: boolean;
  resultCode?: string;
};

export type ToleranceFormState = FormState<ToleranceKey, ToleranceExtras>;

export function createInitialToleranceForm(): ToleranceFormState {
  return {
    status: "editing",
    fields: {
      nominal: emptyField(),
      upper_um: resultField(),
      lower_um: resultField(),
      min_mm: resultField(),
      max_mm: resultField(),
    },
    extras: {
      mode: "hole",
      holeLetter: "H",
      holeGrade: "7",
      shaftLetter: "g",
      shaftGrade: "6",
      options: {
        holes: [],
        shafts: [],
      },
      loadingOptions: true,
      resultCode: undefined,
    },
  };
}

export function buildToleranceFormInput(
  form: ToleranceFormState,
): ToleranceFormInput {
  return {
    mode: form.extras.mode,
    nominal: form.fields.nominal.value,
    holeLetter: form.extras.holeLetter,
    holeGrade: form.extras.holeGrade,
    shaftLetter: form.extras.shaftLetter,
    shaftGrade: form.extras.shaftGrade,
  };
}

export function resultField() {
  return emptyField({ locked: true });
}
