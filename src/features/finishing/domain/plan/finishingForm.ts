// features/finishing/domain/plan/finishingForm.ts

import type { FormState } from "@shared/form/types/forms";
import { emptyField } from "@shared/form/types/fields";

/* ============================================================
   Keys
============================================================ */

export type FinishingKey =
  | "start_diameter"
  | "target_diameter"
  | "cuts"
  | "radial_engagement";

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
      start_diameter: emptyField(),
      target_diameter: emptyField(),
      cuts: emptyField(),
      radial_engagement: emptyField(),
    },
    extras: {
      mode: "Inner",
      planning: "ByCuts",
    },
  };
}