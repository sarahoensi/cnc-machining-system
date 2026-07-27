import { describe, expect, it } from "vitest";

import { emptyField } from "@shared/form/types/fields";
import { getCalculatorFocusIntent } from "./useCalculatorFormNavigation";

const fieldOrder = ["first", "second"] as const;

function createFields() {
  return {
    first: emptyField(),
    second: emptyField(),
  };
}

describe("getCalculatorFocusIntent", () => {
  it("prioritizes inline field errors", () => {
    const fields = createFields();
    fields.second = emptyField({ error: "Invalid value" });

    expect(
      getCalculatorFocusIntent({ fields, formError: "Form error" }, fieldOrder),
    ).toBe("inline-error");
  });

  it("uses form error focus when there are no inline errors", () => {
    expect(
      getCalculatorFocusIntent(
        { fields: createFields(), formError: "Missing input" },
        fieldOrder,
      ),
    ).toBe("form-error");
  });

  it("does not request focus work after a successful calculation", () => {
    expect(getCalculatorFocusIntent({ fields: createFields() }, fieldOrder)).toBe(
      "none",
    );
  });
});
