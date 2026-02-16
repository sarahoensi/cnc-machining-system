import { describe, it, expect } from "vitest";
import {
  applyMachineValue,
  applyUserValue,
  canOverwriteWithMachine,
  clearMachineFields,
} from "./fieldLogic";
import {
  emptyField,
  userField,
  machineField,
  type FieldState,
} from "../../types/fields";

describe("fieldLogic", () => {
  // ----------------------------------------
  // canOverwriteWithMachine
  // ----------------------------------------

  describe("canOverwriteWithMachine", () => {
    it("returns true for empty field", () => {
      const field = emptyField();
      expect(canOverwriteWithMachine(field)).toBe(true);
    });

    it("returns true for machine field", () => {
      const field = machineField("10");
      expect(canOverwriteWithMachine(field)).toBe(true);
    });

    it("returns false for user field with value", () => {
      const field = userField("10");
      expect(canOverwriteWithMachine(field)).toBe(false);
    });

    it("returns true for user field with empty value", () => {
      const field: FieldState = { value: "", source: "user" };
      expect(canOverwriteWithMachine(field)).toBe(true);
    });
  });

  // ----------------------------------------
  // applyMachineValue
  // ----------------------------------------

  describe("applyMachineValue", () => {
    it("applies machine value to empty field", () => {
      const result = applyMachineValue(emptyField(), "42");

      expect(result).toStrictEqual(machineField("42"));
    });

    it("does not overwrite real user value", () => {
      const original = userField("10");
      const result = applyMachineValue(original, "99");

      expect(result).toBe(original);
    });

    it("overwrites existing machine value", () => {
      const original = machineField("10");
      const result = applyMachineValue(original, "99");

      expect(result).toStrictEqual(machineField("99"));
    });
  });

  // ----------------------------------------
  // applyUserValue
  // ----------------------------------------

  describe("applyUserValue", () => {
    it("sets source to user when value is non-empty", () => {
      const result = applyUserValue("123");

      expect(result).toStrictEqual({
        value: "123",
        source: "user",
      });
    });

    it("sets source to empty when value is empty string", () => {
      const result = applyUserValue("");

      expect(result).toStrictEqual(emptyField());
    });
  });

  // ----------------------------------------
  // clearMachineFields
  // ----------------------------------------

  describe("clearMachineFields", () => {
    type Fields = {
      a: FieldState;
      b: FieldState;
      c: FieldState;
    };

    function createFields(): Fields {
      return {
        a: userField("10"),
        b: machineField("20"),
        c: emptyField(),
      };
    }

    it("clears only machine fields", () => {
      const fields = createFields();

      const next = clearMachineFields(fields);

      expect(next).toStrictEqual({
        a: userField("10"),
        b: emptyField(),
        c: emptyField(),
      });
    });

    it("returns same reference if nothing changes", () => {
      const fields: Fields = {
        a: userField("10"),
        b: emptyField(),
        c: emptyField(),
      };

      const next = clearMachineFields(fields);

      expect(next).toBe(fields);
    });
  });
});
