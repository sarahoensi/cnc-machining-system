// features/right_triangle/parseTriangle.ts

import { safeParseDecimal } from "@shared/engine/parsing/decimalParser";
import { TriangleKey } from "./triangleForm";
import { FieldState } from "@shared/types";

export function parseTriangle(
  fields: Record<TriangleKey, FieldState>
): Record<TriangleKey, number> | null {

  const parsed: Partial<Record<TriangleKey, number>> = {};

  for (const key in fields) {
    const value = fields[key as TriangleKey].value;

    if (!value) continue;

    const normalized = safeParseDecimal(value);
    if (normalized == null) return null;

    parsed[key as TriangleKey] = normalized;
  }

  return parsed as Record<TriangleKey, number>;
}