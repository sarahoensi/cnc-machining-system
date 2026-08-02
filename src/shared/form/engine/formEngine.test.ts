import { describe, expect, it, vi } from "vitest";

import type { FormState } from "@shared/form/types/forms";
import { emptyField, resultField, userField } from "@shared/form/types/fields";
import { clearMachineFields, handleCalculateAsync, handleUserEdit } from "./formEngine";

type Key = "input" | "result" | "other";
type Extras = Record<string, never>;

function createForm(): FormState<Key, Extras> {
  return {
    status: "editing",
    fields: {
      input: emptyField(),
      result: resultField(),
      other: emptyField(),
    },
    extras: {},
  };
}

describe("formEngine", () => {
  it("normalizes user edits and clears form errors", () => {
    const form = {
      ...createForm(),
      formError: "Old error",
    };

    const next = handleUserEdit(form, "input", " 12,5 ", [["input"]], []);

    expect(next.status).toBe("editing");
    expect(next.formError).toBeUndefined();
    expect(next.fields.input.value).toBe("12.5");
    expect(next.fields.input.source).toBe("user");
  });

  it("clears machine fields without changing user fields", () => {
    const form = createForm();
    form.fields.input = userField("10", { machineValue: 10 });
    form.fields.result = {
      ...resultField(),
      source: "machine",
      value: "20",
      machineValue: 20,
    };

    const next = clearMachineFields(form.fields);

    expect(next.input.value).toBe("10");
    expect(next.input.machineValue).toBe(10);
    expect(next.result.value).toBe("");
    expect(next.result.source).toBe("empty");
  });

  it("applies solve results as machine values", async () => {
    const form = createForm();
    form.fields.input = userField("4");

    const next = await handleCalculateAsync(
      form,
      () => ({ input: 4 }),
      async () => ({ result: 8 }),
    );

    expect(next.status).toBe("solved");
    expect(next.fields.result.source).toBe("machine");
    expect(next.fields.result.machineValue).toBe(8);
    expect(next.fields.result.value).toBe("8");
  });

  it("returns frontend validation errors without calling solve", async () => {
    const solve = vi.fn(async () => ({ result: 8 }));

    const next = await handleCalculateAsync(
      createForm(),
      () => ({ input: 4 }),
      solve,
      () => ["Input is required"],
    );

    expect(solve).not.toHaveBeenCalled();
    expect(next.status).toBe("editing");
    expect(next.formError).toEqual(["Input is required"]);
  });
});
