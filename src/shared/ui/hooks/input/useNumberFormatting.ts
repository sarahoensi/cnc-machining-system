import { useMemo } from "react";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";

/**
 * Syncs number display formatting with global display settings.
 *
 * Applies presentation rules without mutating raw field values.
 * Returns a formatted string for display only.
 */
export function useNumberFormatting(
  rawValue: string,
  source: "empty" | "user" | "machine"
): string {
  const { decimals } = useDisplaySettings();

  return useMemo(() => {
    // Never format empty values
    if (!rawValue.trim()) return "";

    // Never format while user is typing
    if (source === "user") return rawValue;

    const normalized = rawValue.replace(",", ".");
    const parsed = Number(normalized);

    if (!Number.isFinite(parsed)) {
      return rawValue;
    }

    return parsed.toFixed(decimals);
  }, [rawValue, source, decimals]);
}
