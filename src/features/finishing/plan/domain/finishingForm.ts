// features/finishing/domain/plan/finishingForm.ts

import type { FormState } from "@shared/form/types/forms";
import { emptyField } from "@shared/form/types/fields";

/* ============================================================
   Keys
============================================================ */

export type FinishingKey =
  | "start_diameter_mm"
  | "target_diameter_mm"
  | "cuts"
  | "radial_engagement_mm";

/* ============================================================
   Extras
============================================================ */

export type FinishingExtras = {
  mode: "Inner" | "Outer";
  planning: "ByCuts" | "ByRadialEngagement";
};

/* ============================================================
   Factory
============================================================ */

export function createInitialFinishingForm(): FormState<
  FinishingKey,
  FinishingExtras
> {
  return {
    status: "editing",
    fields: {
      start_diameter_mm: emptyField(),
      target_diameter_mm: emptyField(),
      cuts: emptyField(),
      radial_engagement_mm: emptyField(),
    },
    extras: {
      mode: "Inner",
      planning: "ByCuts",
    },
  };
}

