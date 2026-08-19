import { describe, expect, it } from "vitest";
import { buildLookupIso286ToleranceRequest } from "../domain/buildRequest";
import { createInitialToleranceForm } from "../domain/toleranceForm";
import { parseTolerance } from "../domain/parseTolerance";
import { applyToleranceLetterChange } from "../domain/toleranceSelection";
import { validateToleranceForm } from "../domain/validateToleranceForm";
import { buildToleranceHistoryRow } from "../ui/history/buildToleranceHistoryRow";

describe("tolerance frontend mapping", () => {
  it("creates the initial persisted tolerance form", () => {
    const form = createInitialToleranceForm();

    expect(form.status).toBe("editing");
    expect(form.fields.nominal.value).toBe("");
    expect(form.fields.upper_um.locked).toBe(true);
    expect(form.fields.lower_um.locked).toBe(true);
    expect(form.fields.mid_um.locked).toBe(true);
    expect(form.fields.min_mm.locked).toBe(true);
    expect(form.fields.max_mm.locked).toBe(true);
    expect(form.fields.mid_mm.locked).toBe(true);
    expect(form.fields.hole_letter.value).toBe("H");
    expect(form.fields.hole_grade.value).toBe("7");
    expect(form.fields.shaft_letter.value).toBe("h");
    expect(form.fields.shaft_grade.value).toBe("7");
    expect(form.extras).toMatchObject({
      mode: "hole",
      loadingOptions: true,
    });
  });

  it("parses persisted form state into lookup input", () => {
    const form = createInitialToleranceForm();
    form.fields.nominal.value = " 42,0 ";
    form.extras.mode = "shaft";

    expect(parseTolerance(form.fields, form.extras)).toEqual({
      feature: "shaft",
      nominalMm: 42,
      code: "h7",
    });
  });

  it("rejects unsupported decimal formats consistently", () => {
    const form = createInitialToleranceForm();
    form.fields.nominal.value = "1e3";

    expect(parseTolerance(form.fields, form.extras)).toBeNull();
    expect(validateToleranceForm(form.fields, form.extras)).toEqual([
      "Nominal size must be greater than zero",
    ]);
    expect(
      buildLookupIso286ToleranceRequest({
        nominal: "1e3",
        mode: "hole",
        holeLetter: "H",
        holeGrade: "7",
        shaftLetter: "h",
        shaftGrade: "6",
      }),
    ).toBeNull();
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

  it("keeps the current grade when changing to a class that supports it", () => {
    const form = createInitialToleranceForm();
    form.fields.hole_grade.value = "6";
    form.extras.options = {
      holes: [
        { feature: "hole", zone: "H", grades: [7, 8] },
        { feature: "hole", zone: "JS", grades: [5, 6, 7] },
      ],
      shafts: [],
    };

    const next = applyToleranceLetterChange(form, form.extras.options, "hole", "JS");

    expect(next.fields.hole_letter.value).toBe("JS");
    expect(next.fields.hole_grade.value).toBe("6");
  });

  it("defaults to grade 7 when the selected class does not support the current grade", () => {
    const form = createInitialToleranceForm();
    form.fields.shaft_grade.value = "5";
    form.extras.options = {
      holes: [],
      shafts: [
        { feature: "shaft", zone: "h", grades: [5, 6] },
        { feature: "shaft", zone: "g", grades: [6, 7, 8] },
      ],
    };

    const next = applyToleranceLetterChange(form, form.extras.options, "shaft", "g");

    expect(next.fields.shaft_letter.value).toBe("g");
    expect(next.fields.shaft_grade.value).toBe("7");
  });

  it("rejects missing hole class and invalid nominal size", () => {
    const form = createInitialToleranceForm();
    form.fields.nominal.value = "0";
    form.fields.hole_letter.value = "";
    form.fields.hole_grade.value = "";

    expect(validateToleranceForm(form.fields, form.extras)).toEqual([
      "Hole tolerance letter is required",
      "Hole tolerance grade is required",
      "Nominal size must be greater than zero",
    ]);
  });

  it("rejects missing shaft class and invalid nominal size", () => {
    const form = createInitialToleranceForm();
    form.extras.mode = "shaft";
    form.fields.nominal.value = "0";
    form.fields.shaft_letter.value = "";
    form.fields.shaft_grade.value = "";

    expect(validateToleranceForm(form.fields, form.extras)).toEqual([
      "Shaft tolerance letter is required",
      "Shaft tolerance grade is required",
      "Nominal size must be greater than zero",
    ]);
  });

  it("validates only fields required by the selected single mode", () => {
    const holeForm = createInitialToleranceForm();
    holeForm.fields.nominal.value = "42";
    holeForm.fields.hole_letter.value = "ZA";
    holeForm.fields.hole_grade.value = "8";
    holeForm.fields.shaft_letter.value = "";
    holeForm.fields.shaft_grade.value = "";

    expect(validateToleranceForm(holeForm.fields, holeForm.extras)).toBeNull();

    const shaftForm = createInitialToleranceForm();
    shaftForm.extras.mode = "shaft";
    shaftForm.fields.nominal.value = "42";
    shaftForm.fields.hole_letter.value = "";
    shaftForm.fields.hole_grade.value = "";
    shaftForm.fields.shaft_letter.value = "js";
    shaftForm.fields.shaft_grade.value = "6";

    expect(validateToleranceForm(shaftForm.fields, shaftForm.extras)).toBeNull();
  });

  it("formats hole saved results as compact history rows", () => {
    const form = createInitialToleranceForm();
    form.status = "solved";
    form.extras.mode = "hole";
    form.fields.nominal.value = "6";
    form.fields.hole_letter.value = "H";
    form.fields.hole_grade.value = "7";
    form.fields.lower_um.machineValue = 0;
    form.fields.upper_um.machineValue = 0.012;
    form.fields.min_mm.machineValue = 6;
    form.fields.max_mm.machineValue = 6.012;

    expect(
      buildToleranceHistoryRow(
        {
          id: "hole-row",
          form,
          createdAt: 1,
        },
        3,
      ),
    ).toMatchObject({
      modeLabel: "Hole",
      modeClassName: "tolerance-history-row--hole",
      toleranceClass: "H7",
      nominal: "\u00d86.000 mm",
      deviations: "EI 0.000 / ES +0.012",
    });
  });

  it("formats shaft saved results as compact history rows", () => {
    const form = createInitialToleranceForm();
    form.status = "solved";
    form.extras.mode = "shaft";
    form.fields.nominal.value = "6";
    form.fields.shaft_letter.value = "g";
    form.fields.shaft_grade.value = "6";
    form.fields.lower_um.machineValue = -0.012;
    form.fields.upper_um.machineValue = -0.004;
    form.fields.min_mm.machineValue = 5.988;
    form.fields.max_mm.machineValue = 5.996;

    expect(
      buildToleranceHistoryRow(
        {
          id: "shaft-row",
          form,
          createdAt: 1,
        },
        3,
      ),
    ).toMatchObject({
      modeLabel: "Shaft",
      modeClassName: "tolerance-history-row--shaft",
      toleranceClass: "g6",
      nominal: "\u00d86.000 mm",
      deviations: "ei -0.012 / es -0.004",
    });
  });
});
