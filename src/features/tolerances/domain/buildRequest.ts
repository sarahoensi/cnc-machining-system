import { parseDecimalInput } from "@shared/parsing/decimalParser";
import type { ToleranceMode } from "../api/types";

export type ToleranceFormInput = {
  mode: ToleranceMode;
  nominal: string;
  holeLetter: string;
  holeGrade: string;
  shaftLetter: string;
  shaftGrade: string;
};

function parseNominal(input: ToleranceFormInput) {
  return parseDecimalInput(input.nominal).number;
}

export function buildLookupIso286ToleranceRequest(input: ToleranceFormInput) {
  const nominalMm = parseNominal(input);
  if (nominalMm == null) return null;

  const feature = input.mode;
  const letter = feature === "hole" ? input.holeLetter : input.shaftLetter;
  const grade = feature === "hole" ? input.holeGrade : input.shaftGrade;

  return {
    feature,
    nominalMm,
    code: `${letter}${grade}`.trim(),
  };
}
