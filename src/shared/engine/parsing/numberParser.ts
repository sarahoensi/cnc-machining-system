// shared/engine/parsing/numberParser.ts

/**
 * Parses string into number.
 * Accepts both "," and "." as decimal separator.
 * Returns null if invalid.
 */
export function parseNumber(value: string): number | null {
  const trimmed = value.trim();
  if (!trimmed) return null;

  const normalized = trimmed.replace(",", ".");
  const parsed = Number(normalized);

  return Number.isFinite(parsed) ? parsed : null;
}


/**
 * Parses a record of numeric form fields.
 * Does NOT apply domain constraints.
 */
export function parseNumberFields<
  F extends Record<string, { value: string }>
>(
  fields: F
): {
  input: Partial<Record<keyof F, number>>;
  errors: Partial<Record<keyof F, string>>;
} {
  const input: Partial<Record<keyof F, number>> = {};
  const errors: Partial<Record<keyof F, string>> = {};

  for (const key in fields) {
    const parsed = parseNumber(fields[key].value);

    if (fields[key].value.trim() === "") continue;

    if (parsed === null) {
      errors[key] = "Invalid number";
      continue;
    }

    input[key] = parsed;
  }

  return { input, errors };
}
