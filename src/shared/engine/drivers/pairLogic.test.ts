// shared/core/pairLogic.test.ts

import { describe, it, expect } from "vitest";
import { resolvePairLocks } from "./pairLogic";
import { emptyField, userField, machineField } from "../../types/fields";

type Fields = {
  Vc: any;
  n: any;
  F: any;
  fz: any;
};

const pairs = [
  ["Vc", "n"],
  ["F", "fz"],
] as const;

function createEmpty(): Fields {
  return {
    Vc: emptyField(),
    n: emptyField(),
    F: emptyField(),
    fz: emptyField(),
  };
}

describe("pairLogic", () => {

  it("locks sibling when one side is user", () => {
    const fields = {
      ...createEmpty(),
      Vc: userField("120"),
    };

    const locked = resolvePairLocks(fields, pairs);

    expect(locked).toContain("n");
  });

  it("does not lock sibling if sibling is machine", () => {
    const fields = {
      ...createEmpty(),
      Vc: userField("120"),
      n: machineField("1000"),
    };

    const locked = resolvePairLocks(fields, pairs);

    expect(locked).not.toContain("n");
  });

  it("handles multiple independent pairs", () => {
    const fields = {
      ...createEmpty(),
      Vc: userField("120"),
      F: userField("300"),
    };

    const locked = resolvePairLocks(fields, pairs);

    expect(locked).toContain("n");
    expect(locked).toContain("fz");
  });

});
