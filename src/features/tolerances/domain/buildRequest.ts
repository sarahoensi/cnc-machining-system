export type ToleranceFormInput = {
  nominal: string;
  holeLetter: string;
  holeGrade: string;
  shaftLetter: string;
  shaftGrade: string;
};

export function buildCalculateIso286FitRequest(
  input: ToleranceFormInput,
) {
  return {
    nominal_mm: Number(input.nominal.replace(",", ".")),
    hole: `${input.holeLetter}${input.holeGrade}`.trim(),
    shaft: `${input.shaftLetter}${input.shaftGrade}`.trim(),
  };
}
