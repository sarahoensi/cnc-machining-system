import { describe, expect, it } from "vitest";

import {
  normalizeDecimalInput,
  parseDecimalInput,
  safeParseDecimal,
} from "./decimalParser";

describe("decimalParser", () => {
  it("normalizes whitespace and comma decimal separators", () => {
    expect(normalizeDecimalInput(" 12,5 ")).toBe("12.5");
  });

  it("parses strict decimal values", () => {
    expect(safeParseDecimal("12")).toBe(12);
    expect(safeParseDecimal("-12.5")).toBe(-12.5);
  });

  it("rejects unsupported decimal formats", () => {
    expect(safeParseDecimal("")).toBeNull();
    expect(safeParseDecimal("+12")).toBeNull();
    expect(safeParseDecimal("1e3")).toBeNull();
    expect(safeParseDecimal("12.")).toBeNull();
  });

  it("returns normalized text and parsed number together", () => {
    expect(parseDecimalInput(" 3,25 ")).toEqual({
      normalized: "3.25",
      number: 3.25,
    });
  });
});
