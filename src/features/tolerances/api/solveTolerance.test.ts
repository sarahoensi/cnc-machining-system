import { describe, expect, it } from "vitest";
import { buildLookupIso286ToleranceRequest } from "../domain/buildRequest";
import {
  buildToleranceFormInput,
  createInitialToleranceForm,
  migrateToleranceForm,
} from "../domain/toleranceForm";
import { validateToleranceForm } from "../domain/validateToleranceForm";
import { buildToleranceHistoryRow } from "../ui/toleranceHistoryRows";

describe("tolerance frontend mapping", () => {
  it("creates the initial persisted tolerance form", () => {
    const form = createInitialToleranceForm();

    expect(form.status).toBe("editing");
    expect(form.fields.nominal.value).toBe("");
    expect(form.fields.upper_um.locked).toBe(true);
    expect(form.fields.lower_um.locked).toBe(true);
    expect(form.fields.min_mm.locked).toBe(true);
    expect(form.fields.max_mm.locked).toBe(true);
    expect(form.fields.hole_letter.value).toBe("H");
    expect(form.fields.hole_grade.value).toBe("7");
    expect(form.fields.shaft_letter.value).toBe("g");
    expect(form.fields.shaft_grade.value).toBe("6");
    expect(form.extras).toMatchObject({
      mode: "hole",
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

  it("keeps solved result fields unlocked during migration", () => {
    const form = createInitialToleranceForm();
    form.status = "solved";
    form.fields.upper_um = {
      ...form.fields.upper_um,
      value: "25",
      source: "machine",
      machineValue: 25,
      locked: false,
    };
    form.fields.lower_um = {
      ...form.fields.lower_um,
      value: "0",
      source: "machine",
      machineValue: 0,
      locked: false,
    };
    form.fields.min_mm = {
      ...form.fields.min_mm,
      value: "42",
      source: "machine",
      machineValue: 42,
      locked: false,
    };
    form.fields.max_mm = {
      ...form.fields.max_mm,
      value: "42.025",
      source: "machine",
      machineValue: 42.025,
      locked: false,
    };

    const migrated = migrateToleranceForm(form);

    expect(migrated.fields.upper_um.locked).toBe(false);
    expect(migrated.fields.lower_um.locked).toBe(false);
    expect(migrated.fields.min_mm.locked).toBe(false);
    expect(migrated.fields.max_mm.locked).toBe(false);
  });

  it("unlocks persisted solved machine result fields during migration", () => {
    const form = createInitialToleranceForm();
    form.status = "solved";

    for (const key of ["upper_um", "lower_um", "min_mm", "max_mm"] as const) {
      form.fields[key] = {
        ...form.fields[key],
        value: "1",
        source: "machine",
        machineValue: 1,
        locked: true,
      };
    }

    const migrated = migrateToleranceForm(form);

    expect(migrated.fields.upper_um.locked).toBe(false);
    expect(migrated.fields.lower_um.locked).toBe(false);
    expect(migrated.fields.min_mm.locked).toBe(false);
    expect(migrated.fields.max_mm.locked).toBe(false);
  });

  it("converts persisted solved deviation results from micrometers to millimeters", () => {
    const form = createInitialToleranceForm();
    form.status = "solved";
    form.extras = ({
      ...form.extras,
      deviationUnit: undefined,
    } as unknown) as typeof form.extras;
    form.fields.upper_um = {
      ...form.fields.upper_um,
      value: "25",
      source: "machine",
      machineValue: 25,
      locked: true,
    };
    form.fields.lower_um = {
      ...form.fields.lower_um,
      value: "-9",
      source: "machine",
      machineValue: -9,
      locked: true,
    };

    const migrated = migrateToleranceForm(form);

    expect(migrated.fields.upper_um.value).toBe("0.025");
    expect(migrated.fields.upper_um.machineValue).toBe(0.025);
    expect(migrated.fields.lower_um.value).toBe("-0.009");
    expect(migrated.fields.lower_um.machineValue).toBe(-0.009);
    expect(migrated.extras.deviationUnit).toBe("mm");
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
