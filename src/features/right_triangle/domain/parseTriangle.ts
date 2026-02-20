// features/right_triangle/parseTriangle.ts

import { safeParseDecimal } from "@shared/engine/parsing/decimalParser";
import { TriangleKey } from "./triangleForm";
import { FieldState } from "@shared/types";

export function parseTriangle(
  fields: Record<TriangleKey, FieldState>
): Partial<Record<TriangleKey, number>> | null {

  const parsed: Partial<Record<TriangleKey, number>> = {};

  for (const key in fields) {
    const k = key as TriangleKey;
    const value = fields[k].value;

    if (!value) continue;

    const normalized = safeParseDecimal(value);
    if (normalized == null) return null;

    parsed[k] = normalized;
  }

  return parsed;
}