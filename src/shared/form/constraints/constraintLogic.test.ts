// shared/form/constraints/constraintLogic.test.ts

import { describe, it, expect } from "vitest";
import { evaluateConstraints } from "./constraintLogic";
import { emptyField, userField } from "@shared/form/types/fields";

type Keys = "a" | "b" | "c";

const validSets: readonly (readonly Keys[])[] = [
  ["a", "b"],
  ["b", "c"],
];

function createInitial() {
  return {
    a: emptyField(),
    b: emptyField(),
    c: emptyField(),
  };
}

describe("constraintLogic", () => {
  it("does not lock fields when there is no user input", () => {
    const fields = createInitial();

    const result = evaluateConstraints(fields, validSets, null);

    expect(result.fields.a.locked).toBe(false);
    expect(result.fields.b.locked).toBe(false);
    expect(result.fields.c.locked).toBe(false);
  });

  it("single user input restricts allowed keys", () => {
    const fields = createInitial();
    fields.a = userField("10");

    const result = evaluateConstraints(fields, validSets, "a");

    expect(result.fields.a.locked).toBe(false);
    expect(result.fields.b.locked).toBe(false);
    expect(result.fields.c.locked).toBe(true);

    expect(result.fields.c.value).toBe("");
  });

  it("lets editedKey win conflicts", () => {
    const fields = createInitial();
    fields.a = userField("10");
    fields.c = userField("20"); // not compatible

    const result = evaluateConstraints(fields, validSets, "c");

    expect(result.fields.c.source).toBe("user");
    expect(result.fields.a.source).toBe("empty");
  });

  it("locked fields are always empty", () => {
    const fields = createInitial();
    fields.a = userField("10");

    const result = evaluateConstraints(fields, validSets, "a");

    expect(result.fields.c.locked).toBe(true);
    expect(result.fields.c.value).toBe("");
    expect(result.fields.c.source).toBe("empty");
  });
});
