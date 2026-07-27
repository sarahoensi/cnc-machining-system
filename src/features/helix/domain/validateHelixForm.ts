// features/helix/domain/validateHelixForm.ts

import { FieldState } from "@shared/form";
import { HelixKey } from "./helixForm";

export function validateHelixForm(
  fields: Record<HelixKey, FieldState>,
): string[] | null {
  const errors: string[] = [];

  if (!fields.diameter.value) {
    errors.push("Diameter is required");
  }

  if (!fields.tool_diameter.value) {
    errors.push("Tool diameter is required");
  }

  if (!fields.pitch.value && !fields.angle.value) {
    errors.push("Provide either pitch or angle");
  }

  return errors.length > 0 ? errors : null;
}
