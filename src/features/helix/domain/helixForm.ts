// features/helix/domain/helixForm.ts

import type { FormState } from "@shared/form/types/forms";
import { emptyField } from "@shared/form/types/fields";

/* ============================================================
   Keys
============================================================ */

export type HelixKey = "diameter" | "tool_diameter" | "pitch" | "angle";

/* ============================================================
   Extras
============================================================ */

export type HelixExtras = {
  mode: "Inner" | "Outer";
};

/* ============================================================
   Factory
============================================================ */

export function createInitialHelixForm(): FormState<HelixKey, HelixExtras> {
  return {
    status: "editing",
    fields: {
      diameter: emptyField(),
      tool_diameter: emptyField(),
      pitch: emptyField(),
      angle: emptyField(),
    },
    extras: {
      mode: "Outer",
    },
  };
}
