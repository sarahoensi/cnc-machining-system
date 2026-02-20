import type { FormState } from "@shared/types/forms";
import { emptyField } from "@shared/types/fields";

/* ============================================================
   Keys
============================================================ */

export type HelixKey =
  | "diameter"
  | "toolDiameter"
  | "pitch"
  | "angle"
  | "effectiveDiameter"
  | "circumference";

/* ============================================================
   Extras
============================================================ */

export type HelixExtras = {
  mode: "Inner" | "Outer";
};

/* ============================================================
   Factory
============================================================ */

export function createInitialHelixForm(): FormState<
  HelixKey,
  HelixExtras
> {
  return {
    status: "editing",
    fields: {
      diameter: emptyField(),
      toolDiameter: emptyField(),
      pitch: emptyField(),
      angle: emptyField(),
      effectiveDiameter: emptyField(),
      circumference: emptyField(),
    },
    extras: {
      mode: "Outer",
    },
  };
}