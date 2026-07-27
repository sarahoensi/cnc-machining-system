// features/helix/domain/parseHelix.ts

import { safeParseDecimal } from "@shared/parsing/decimalParser";
import { FieldState } from "@shared/form/types";
import { HelixKey } from "./helixForm";

export function parseHelix(
  fields: Record<HelixKey, FieldState>,
): Partial<Record<HelixKey, number>> | null {
  const parsed: Partial<Record<HelixKey, number>> = {};

  for (const key in fields) {
    const k = key as HelixKey;
    const value = fields[k].value;

    if (!value) continue;

    const normalized = safeParseDecimal(value);
    if (normalized == null) return null;

    parsed[k] = normalized;
  }

  return parsed;
}
