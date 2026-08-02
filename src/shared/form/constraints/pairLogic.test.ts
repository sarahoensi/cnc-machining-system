// shared/core/pairLogic.test.ts

import { describe, it, expect } from "vitest";
import { applyPairLogic } from "./pairLogic";
import { emptyField, userField } from "@shared/form/types/fields";

type Keys = "x" | "y";

const pairs: readonly (readonly [Keys, Keys])[] = [["x", "y"]];

function createInitial() {
  return {
    x: emptyField(),
    y: emptyField(),
  };
}

describe("pairLogic", () => {
  it("single user driver locks the other", () => {
    const fields = createInitial();
    fields.x = userField("5");

    const result = applyPairLogic(fields, pairs, "x", "editing");

    expect(result.y.locked).toBe(true);
    expect(result.y.value).toBe("");
  });

  it("lets editedKey win conflicts", () => {
    const fields = createInitial();
    fields.x = userField("5");
    fields.y = userField("10");

    const result = applyPairLogic(fields, pairs, "y", "editing");

    expect(result.y.source).toBe("user");
    expect(result.x.source).toBe("empty");
    expect(result.x.locked).toBe(true);
  });

  it("does nothing in solved mode", () => {
    const fields = createInitial();
    fields.x = userField("5");

    const result = applyPairLogic(fields, pairs, "x", "solved");

    expect(result.y.locked).toBe(false);
  });
});
