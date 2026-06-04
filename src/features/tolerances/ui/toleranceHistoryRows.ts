import { formatNumber } from "@shared/ui/format/formatNumber";

import type { SavedResultEntry } from "@shared/savedResults";
import type { ToleranceFormState } from "../domain/toleranceForm";

export type ToleranceHistoryRow = {
  id: string;
  mode: "hole" | "shaft";
  modeLabel: "Hole" | "Shaft";
  modeClassName: "tolerance-history-row--hole" | "tolerance-history-row--shaft";
  symbol: string;
  toleranceClass: string;
  nominal: string;
  deviations: string;
};

export function buildToleranceHistoryRow(
  entry: SavedResultEntry<ToleranceFormState>,
  decimals: number,
): ToleranceHistoryRow {
  const { form } = entry;
  const mode = form.extras.mode;
  const isHole = mode === "hole";
  const letterKey = isHole ? "hole_letter" : "shaft_letter";
  const gradeKey = isHole ? "hole_grade" : "shaft_grade";
  const lowerLabel = isHole ? "EI" : "ei";
  const upperLabel = isHole ? "ES" : "es";

  return {
    id: entry.id,
    mode,
    modeLabel: isHole ? "Hole" : "Shaft",
    modeClassName: isHole
      ? "tolerance-history-row--hole"
      : "tolerance-history-row--shaft",
    symbol: isHole ? "\u25cb" : "\u25cf",
    toleranceClass:
      `${form.fields[letterKey].value}${form.fields[gradeKey].value}`.trim() ||
      "-",
    nominal: `\u00d8${formatFieldValue(form.fields.nominal, decimals)} mm`,
    deviations: `${lowerLabel} ${formatSignedFieldValue(
      form.fields.lower_um,
      decimals,
    )} / ${upperLabel} ${formatSignedFieldValue(
      form.fields.upper_um,
      decimals,
    )}`,
  };
}

function formatFieldValue(
  field: ToleranceFormState["fields"][keyof ToleranceFormState["fields"]],
  decimals: number,
) {
  const value = getNumericFieldValue(field);
  return value == null ? "-" : formatNumber(value, decimals);
}

function formatSignedFieldValue(
  field: ToleranceFormState["fields"][keyof ToleranceFormState["fields"]],
  decimals: number,
) {
  const value = getNumericFieldValue(field);
  if (value == null) return "-";

  const formatted = formatNumber(value, decimals);
  return value > 0 ? `+${formatted}` : formatted;
}

function getNumericFieldValue(
  field: ToleranceFormState["fields"][keyof ToleranceFormState["fields"]],
) {
  const rawValue = field.machineValue ?? field.value;
  if (rawValue == null || rawValue === "") return null;

  const numericValue =
    typeof rawValue === "number" ? rawValue : Number(rawValue.replace(",", "."));

  return Number.isNaN(numericValue) ? null : numericValue;
}
