// features/cutting_data/domain/parseCuttingData.ts

import { safeParseDecimal } from "@shared/parsing/decimalParser";
import { FieldState } from "@shared/form/types";
import { CuttingDataKey } from "./cuttingDataForm";

export function parseCuttingData(
  fields: Record<CuttingDataKey, FieldState>,
): Partial<Record<CuttingDataKey, number>> | null {
  const parsed: Partial<Record<CuttingDataKey, number>> = {};

  for (const key in fields) {
    const k = key as CuttingDataKey;
    const value = fields[k].value;

    if (!value) continue;

    const normalized = safeParseDecimal(value);
    if (normalized == null) return null;

    parsed[k] = normalized;
  }

  return parsed;
}
