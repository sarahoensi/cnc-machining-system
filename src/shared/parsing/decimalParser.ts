// shared/parsing/decimalParser.ts

// Strict decimal format:
// - optional leading "-"
// - digits
// - optional "." + digits
// - NO scientific notation
// - NO plus sign
const STRICT_DECIMAL = /^-?\d+(\.\d+)?$/;

/**
 * Parses normalized decimal string (must use "." as separator).
 * Returns null if invalid.
 */
export function safeParseDecimal(value: string): number | null {
  if (!STRICT_DECIMAL.test(value)) return null;

  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : null;
}

/**
 * Normalizes user input:
 * - trims
 * - replaces "," with "."
 */
export function normalizeDecimalInput(value: string): string {
  return value.trim().replace(",", ".");
}


export function parseDecimalInput(value: string): {
  normalized: string;
  number: number | null;
} {
  const normalized = normalizeDecimalInput(value);
  const parsed = safeParseDecimal(normalized);

  return {
    normalized,
    number: parsed,
  };
}
