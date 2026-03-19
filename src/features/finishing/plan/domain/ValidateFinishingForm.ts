// features/finishing/domain/plan/validateFinishingForm.ts

import type { FinishingKey } from "./finishingForm";
import type { FieldState } from "@shared/form/types/fields";

export function validateFinishingForm(
  fields: Record<FinishingKey, FieldState>
): string[] | null {

  const errors: string[] = [];

  const start = fields.start_diameter_mm.value;
  const target = fields.target_diameter_mm.value;
  const cuts = fields.cuts.value;
  const engagement = fields.radial_engagement_mm.value;

  if (!start || !target) {
    errors.push("Start and target diameter are required");
  }

  if (!cuts && !engagement) {
    errors.push("Provide either cuts or radial engagement");
  }

  return errors.length > 0 ? errors : null;
}