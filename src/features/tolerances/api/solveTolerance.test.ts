import { describe, expect, it } from "vitest";
import { buildLookupIso286ToleranceRequest } from "../domain/buildRequest";
import {
  buildToleranceFormInput,
  createInitialToleranceForm,
} from "../domain/toleranceForm";
import { validateToleranceForm } from "../domain/validateToleranceForm";

describe("tolerance frontend mapping", () => {
  it("creates the initial persisted tolerance form", () => {
    const form = createInitialToleranceForm();

    expect(form.status).toBe("editing");
    expect(form.fields.nominal.value).toBe("");
    expect(form.fields.upper_um.locked).toBe(true);
    expect(form.fields.lower_um.locked).toBe(true);
    expect(form.fields.min_mm.locked).toBe(true);
    expect(form.fields.max_mm.locked).toBe(true);
    expect(form.extras).toMatchObject({
      mode: "hole",
      holeLetter: "H",
      holeGrade: "7",
      shaftLetter: "g",
      shaftGrade: "6",
      loadingOptions: true,
    });
  });

  it("maps persisted form state into lookup input", () => {
    const form = createInitialToleranceForm();
    form.fields.nominal.value = "42,0";
    form.extras.mode = "shaft";

    expect(buildToleranceFormInput(form)).toEqual({
      mode: "shaft",
      nominal: "42,0",
      holeLetter: "H",
      holeGrade: "7",
      shaftLetter: "g",
      shaftGrade: "6",
    });

    expect(
      buildLookupIso286ToleranceRequest(buildToleranceFormInput(form)),
    ).toEqual({
      feature: "shaft",
      nominalMm: 42,
      code: "g6",
    });
  });

  it("maps form values into single tolerance lookup requests", () => {
    expect(
      buildLookupIso286ToleranceRequest({
        nominal: "42,0",
        mode: "hole",
        holeLetter: "JS",
        holeGrade: "7",
        shaftLetter: "h",
        shaftGrade: "6",
      }),
    ).toEqual({
      feature: "hole",
      nominalMm: 42,
      code: "JS7",
    });

    expect(
      buildLookupIso286ToleranceRequest({
        nominal: "42,0",
        mode: "shaft",
        holeLetter: "JS",
        holeGrade: "7",
        shaftLetter: "h",
        shaftGrade: "6",
      }),
    ).toEqual({
      feature: "shaft",
      nominalMm: 42,
      code: "h6",
    });
  });

  it("rejects missing hole class and invalid nominal size", () => {
    expect(
      validateToleranceForm({
        nominal: "0",
        mode: "hole",
        holeLetter: "",
        holeGrade: "",
        shaftLetter: "",
        shaftGrade: "",
      }),
    ).toEqual({
      holeGrade: "Hole tolerance grade is required",
      holeLetter: "Hole tolerance letter is required",
      nominal: "Nominal size must be greater than zero",
    });
  });

  it("rejects missing shaft class and invalid nominal size", () => {
    expect(
      validateToleranceForm({
        nominal: "0",
        mode: "shaft",
        holeLetter: "",
        holeGrade: "",
        shaftLetter: "",
        shaftGrade: "",
      }),
    ).toEqual({
      nominal: "Nominal size must be greater than zero",
      shaftGrade: "Shaft tolerance grade is required",
      shaftLetter: "Shaft tolerance letter is required",
    });
  });

  it("validates only fields required by the selected single mode", () => {
    expect(
      validateToleranceForm({
        nominal: "42",
        mode: "hole",
        holeLetter: "ZA",
        holeGrade: "8",
        shaftLetter: "",
        shaftGrade: "",
      }),
    ).toEqual({});

    expect(
      validateToleranceForm({
        nominal: "42",
        mode: "shaft",
        holeLetter: "",
        holeGrade: "",
        shaftLetter: "js",
        shaftGrade: "6",
      }),
    ).toEqual({});
  });
});
