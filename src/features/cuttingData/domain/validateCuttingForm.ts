// src/features/cuttingData/domain/validateCuttingForm.ts

//

import type { CuttingDataKey } from "./cuttingDataForm";
import type { FieldState } from "@shared/form/types/fields";

export function validateCuttingDataForm(
  fields: Record<CuttingDataKey, FieldState>,
): string[] | null {
  const errors: string[] = [];

  const diameter = fields.diameter.value;
  const rpm = fields.rpm.value;
  const vc = fields.cutting_speed.value;

  const teeth = fields.teeth.value;
  const chip = fields.chip_load.value;
  const feed = fields.feed_rate.value;

  // 🔹 Diameter alltid nødvendig
  if (!diameter) {
    errors.push("Diameter is required");
  }

  // 🔹 Minimum for å kunne gjøre noe nyttig
  const hasRotation = rpm || vc;
  const hasFeed = teeth && (chip || feed);

  if (!hasRotation && !hasFeed) {
    errors.push("Provide cutting speed or rpm, or feed inputs");
  }

  return errors.length > 0 ? errors : null;
}
