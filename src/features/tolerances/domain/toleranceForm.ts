// features/tolerances/domain/toleranceForm.ts

import type { FormState } from "@shared/form/types/forms";
import { emptyField, resultField, userField } from "@shared/form/types/fields";

import type { ToleranceMode, ToleranceOptionsResponse } from "../api/types";

export type ToleranceKey =
  | "nominal"
  | "hole_letter"
  | "hole_grade"
  | "shaft_letter"
  | "shaft_grade"
  | "upper_um"
  | "lower_um"
  | "mid_um"
  | "min_mm"
  | "max_mm"
  | "mid_mm";

export type ToleranceExtras = {
  mode: ToleranceMode;
  options: ToleranceOptionsResponse;
  loadingOptions: boolean;
  deviationUnit: "mm";
};

export type ToleranceFormState = FormState<ToleranceKey, ToleranceExtras>;

export function createInitialToleranceForm(): ToleranceFormState {
  return {
    status: "editing",
    fields: {
      nominal: emptyField(),
      hole_letter: userField("H"),
      hole_grade: userField("7"),
      shaft_letter: userField("h"),
      shaft_grade: userField("7"),
      upper_um: resultField(),
      lower_um: resultField(),
      mid_um: resultField(),
      min_mm: resultField(),
      max_mm: resultField(),
      mid_mm: resultField(),
    },
    extras: {
      mode: "hole",
      options: {
        holes: [],
        shafts: [],
      },
      loadingOptions: true,
      deviationUnit: "mm",
    },
  };
}
