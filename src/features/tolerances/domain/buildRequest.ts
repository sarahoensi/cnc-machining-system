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
  return Number(input.nominal.replace(",", "."));
}

export function buildLookupIso286ToleranceRequest(
  input: ToleranceFormInput,
) {
  const feature = input.mode;
  const letter = feature === "hole" ? input.holeLetter : input.shaftLetter;
  const grade = feature === "hole" ? input.holeGrade : input.shaftGrade;

  return {
    feature,
    nominalMm: parseNominal(input),
    code: `${letter}${grade}`.trim(),
  };
}
