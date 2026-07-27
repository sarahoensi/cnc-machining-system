// features/finishing/domain/plan/parseFinishingPlan.ts

import { safeParseDecimal } from "@shared/parsing/decimalParser";
import type { FieldState } from "@shared/form/types/fields";
import type { FinishingKey } from "./finishingForm";

export function parseFinishingPlan(
  fields: Record<FinishingKey, FieldState>,
): Partial<Record<FinishingKey, number>> | null {
  const parsed: Partial<Record<FinishingKey, number>> = {};

  for (const key in fields) {
    const k = key as FinishingKey;
    const value = fields[k].value;

    if (!value) continue;

    const normalized = safeParseDecimal(value);
    if (normalized == null) return null;

    parsed[k] = normalized;
  }

  return parsed;
}
