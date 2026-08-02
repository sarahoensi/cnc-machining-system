// features/cutting_data/domain/cuttingDataForm.ts

import type { FormState } from "@shared/form/types/forms";
import { emptyField } from "@shared/form/types/fields";

/* ============================================================
   Keys
============================================================ */

export type CuttingDataKey =
  | "diameter" // D
  | "cutting_speed" // Vc
  | "rpm" // n
  | "teeth" // z
  | "feed_rate" // F
  | "chip_load"; // fz

/* ============================================================
   Extras
============================================================ */

export type CuttingDataExtras = {};

/* ============================================================
   Factory
============================================================ */

export function createInitialCuttingDataForm(): FormState<
  CuttingDataKey,
  CuttingDataExtras
> {
  return {
    status: "editing",
    fields: {
      diameter: emptyField(),
      teeth: emptyField(),
      cutting_speed: emptyField(),
      rpm: emptyField(),
      feed_rate: emptyField(),
      chip_load: emptyField(),
    },
    extras: {},
  };
}
