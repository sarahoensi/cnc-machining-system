import { describe, expect, it } from "vitest";

import { userField } from "@shared/form/types/fields";

import { createInitialCuttingDataForm } from "./cuttingDataForm";
import { validateCuttingDataForm } from "./validateCuttingForm";

describe("validateCuttingDataForm", () => {
  it("explains the minimum field sets when the form is empty", () => {
    const form = createInitialCuttingDataForm();

    expect(validateCuttingDataForm(form.fields)).toEqual([
      "Fill in D and either Vc or n.",
      "For feed, fill in z and either F or fz.",
    ]);
  });

  it("asks for the missing rotation counterpart by field name", () => {
    const form = createInitialCuttingDataForm();
    form.fields.diameter = userField("10");

    expect(validateCuttingDataForm(form.fields)).toEqual([
      "Fill in either cutting speed Vc or rotations n.",
    ]);
  });

  it("asks for rpm context when feed inputs cannot be solved yet", () => {
    const form = createInitialCuttingDataForm();
    form.fields.teeth = userField("4");
    form.fields.chip_load = userField("0.05");

    expect(validateCuttingDataForm(form.fields)).toEqual([
      "Tool diameter D must be filled in.",
      "Fill in either cutting speed Vc or rotations n.",
      "To calculate feed, fill in rotations n or cutting speed Vc.",
    ]);
  });
});
