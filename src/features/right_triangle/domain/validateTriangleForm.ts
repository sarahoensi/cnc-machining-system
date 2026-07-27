// features/right_triangle/domain/validateTriangleForm.ts

import type { TriangleKey } from "./triangleForm";
import type { FieldState } from "@shared/form/types/fields";

export function validateTriangleForm(
  fields: Record<TriangleKey, FieldState>,
): string[] | null {
  const values = Object.values(fields)
    .map((f) => f.value)
    .filter((v) => v !== "");

  if (values.length < 2) {
    return [
      values.length === 0 ? "Provide input values" : "Provide at least two values",
    ];
  }

  return null;
}
