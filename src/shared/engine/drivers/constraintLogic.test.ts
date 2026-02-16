// shared/core/constraintLogic.test.ts

import { describe, it, expect } from "vitest";
import {
  resolveActiveSet,
  resolveLockedKeys,
  applyConstraintSwitch,
} from "./constraintLogic";

import { emptyField, userField } from "../../types/fields";

type Fields = {
  a: any;
  b: any;
  c: any;
};

const validSets = [
  ["a", "b"],
  ["a", "c"],
] as const;

function createEmpty(): Fields {
  return {
    a: emptyField(),
    b: emptyField(),
    c: emptyField(),
  };
}

describe("constraintLogic", () => {

  describe("resolveActiveSet", () => {

    it("returns null when no user fields", () => {
      const fields = createEmpty();
      expect(resolveActiveSet(fields, validSets)).toBeNull();
    });

    it("returns matching set when valid combination", () => {
      const fields = {
        ...createEmpty(),
        a: userField("10"),
        b: userField("20"),
      };

      expect(resolveActiveSet(fields, validSets)).toEqual(["a", "b"]);
    });

    it("returns null when combination is invalid", () => {
      const fields = {
        ...createEmpty(),
        b: userField("10"),
        c: userField("20"),
      };

      expect(resolveActiveSet(fields, validSets)).toBeNull();
    });

  });

  describe("resolveLockedKeys", () => {

    it("locks keys not in active set", () => {
      const fields = {
        ...createEmpty(),
        a: userField("10"),
        b: userField("20"),
      };

      const locked = resolveLockedKeys(fields, validSets);

      expect(locked).toContain("c");
      expect(locked).not.toContain("a");
      expect(locked).not.toContain("b");
    });

  });

  describe("applyConstraintSwitch", () => {

    it("clears conflicting user fields in same set", () => {
      const fields = {
        ...createEmpty(),
        a: userField("10"),
        b: userField("20"),
      };

      const next = applyConstraintSwitch(fields, "a", validSets);

      expect(next.b.value).toBe("");
      expect(next.b.source).toBe("empty");
    });

    it("does nothing if edited key not in any set", () => {
      const fields = {
        ...createEmpty(),
        c: userField("5"),
      };

      const next = applyConstraintSwitch(fields, "c", validSets);

      expect(next).toStrictEqual(fields);
;
    });

  });

});
