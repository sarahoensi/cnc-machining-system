import { describe, expect, it } from "vitest";
import { getTauriCommandError } from "./tauriError";

describe("getTauriCommandError", () => {
  it("returns structured errors directly", () => {
    const result = getTauriCommandError({
      message: "Validation failed",
      fieldErrors: [{ field: "rpm", code: "invalid", message: "must be > 0" }],
    });

    expect(result).toEqual({
      message: "Validation failed",
      fieldErrors: [{ field: "rpm", code: "invalid", message: "must be > 0" }],
    });
  });

  it("parses JSON encoded message errors", () => {
    const result = getTauriCommandError({
      message: JSON.stringify({
        message: "Validation failed",
        fieldErrors: [{ field: "diameter", code: "invalid", message: "bad value" }],
      }),
    });

    expect(result?.fieldErrors?.[0].field).toBe("diameter");
  });

  it("returns null for unknown shapes", () => {
    expect(getTauriCommandError({ message: "plain text only" })).toEqual({
      message: "plain text only",
      fieldErrors: undefined,
    });
    expect(getTauriCommandError(null)).toBeNull();
  });
});
