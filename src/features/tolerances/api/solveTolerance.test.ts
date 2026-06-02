import { describe, expect, it } from "vitest";
import { buildCalculateIso286FitRequest } from "../domain/buildRequest";
import { validateToleranceForm } from "../domain/validateToleranceForm";

describe("tolerance frontend mapping", () => {
  it("maps form values into backend request keys", () => {
    const request = buildCalculateIso286FitRequest({
      nominal: "42,0",
      holeLetter: "H",
      holeGrade: "7",
      shaftLetter: "g",
      shaftGrade: "6",
    });

    expect(request).toEqual({
      nominal_mm: 42,
      hole: "H7",
      shaft: "g6",
    });
  });

  it("rejects missing classes and invalid nominal size", () => {
    expect(
      validateToleranceForm({
        nominal: "0",
        holeLetter: "",
        holeGrade: "",
        shaftLetter: "",
        shaftGrade: "",
      }),
    ).toEqual({
      holeGrade: "Hole tolerance grade is required",
      holeLetter: "Hole tolerance letter is required",
      nominal: "Nominal size must be greater than zero",
      shaftGrade: "Shaft tolerance grade is required",
      shaftLetter: "Shaft tolerance letter is required",
    });
  });
});
