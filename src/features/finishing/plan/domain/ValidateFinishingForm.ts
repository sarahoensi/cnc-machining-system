// features/finishing/domain/plan/validateFinishingForm.ts

import type { FinishingKey } from "./finishingForm";
import type { FieldState } from "@shared/form/types/fields";

export function validateFinishingForm(
  fields: Record<FinishingKey, FieldState>
): string[] | null {

  const errors: string[] = [];

  const cuts = fields.cuts.value;
  const engagement = fields.radial_engagement_mm.value;

  if (!cuts && !engagement) {
    errors.push("Provide either cuts or radial engagement");
  }

  return errors.length > 0 ? errors : null;
}
