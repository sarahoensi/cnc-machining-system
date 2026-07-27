// src/features/cylinder_weight/ui/materials/sortMaterials.ts

import { CylinderMaterial } from "./types";

type MaterialSortToken =
  | { kind: "numeric"; numericValue: number; fallback: string }
  | { kind: "text"; textValue: string; fallback: string };

const NUMERIC_ONLY_PATTERN = /^[+-]?\d+(?:[.,]\d+)?$/;
const LEADING_NUMERIC_PREFIX_BEFORE_TEXT_PATTERN = /^[+-]?[\d.,]+\s*(?=\p{L})/u;

function toSortToken(name: string): MaterialSortToken {
  const trimmed = name.trim();

  if (NUMERIC_ONLY_PATTERN.test(trimmed)) {
    const numericValue = Number(trimmed.replace(",", "."));
    return {
      kind: "numeric",
      numericValue,
      fallback: trimmed,
    };
  }

  const withoutNumericPrefix = trimmed
    .replace(LEADING_NUMERIC_PREFIX_BEFORE_TEXT_PATTERN, "")
    .trim();

  if (withoutNumericPrefix.length > 0 && withoutNumericPrefix !== trimmed) {
    return {
      kind: "text",
      textValue: withoutNumericPrefix,
      fallback: trimmed,
    };
  }

  return {
    kind: "text",
    textValue: trimmed,
    fallback: trimmed,
  };
}

function compareText(left: string, right: string): number {
  return left.localeCompare(right, undefined, { sensitivity: "base" });
}

export function sortCylinderMaterials(rows: CylinderMaterial[]): CylinderMaterial[] {
  return rows
    .map((row, index) => ({
      row,
      index,
      token: toSortToken(row.name),
    }))
    .sort((a, b) => {
      const left = a.token;
      const right = b.token;

      if (left.kind === "numeric" && right.kind === "numeric") {
        if (left.numericValue !== right.numericValue) {
          return left.numericValue - right.numericValue;
        }
        const byFallback = compareText(left.fallback, right.fallback);
        if (byFallback !== 0) return byFallback;
        return a.index - b.index;
      }

      if (left.kind === "text" && right.kind === "text") {
        const byText = compareText(left.textValue, right.textValue);
        if (byText !== 0) return byText;
        const byFallback = compareText(left.fallback, right.fallback);
        if (byFallback !== 0) return byFallback;
        return a.index - b.index;
      }

      if (left.kind === "text" && right.kind === "numeric") {
        const byMixed = compareText(left.textValue, right.fallback);
        if (byMixed !== 0) return byMixed;
        return a.index - b.index;
      }

      const rightTextValue = right.kind === "text" ? right.textValue : right.fallback;
      const byMixed = compareText(left.fallback, rightTextValue);
      if (byMixed !== 0) return byMixed;
      return a.index - b.index;
    })
    .map((entry) => entry.row);
}
