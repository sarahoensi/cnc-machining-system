import { describe, expect, it, vi } from "vitest";
import { resolveInitialForm } from "./FormStateProvider";

describe("resolveInitialForm", () => {
  it("uses existing form without re-initializing", () => {
    const createInitial = vi.fn(() => ({ a: 1 }));
    const current = { a: 2 };

    const result = resolveInitialForm(current, undefined, createInitial);

    expect(result.value).toEqual(current);
    expect(result.shouldHydrate).toBe(false);
    expect(createInitial).not.toHaveBeenCalled();
  });

  it("creates and marks hydration when form is missing", () => {
    const createInitial = vi.fn(() => ({ a: 1 }));

    const result = resolveInitialForm(undefined, undefined, createInitial);

    expect(result.value).toEqual({ a: 1 });
    expect(result.shouldHydrate).toBe(true);
    expect(createInitial).toHaveBeenCalledTimes(1);
  });

  it("reuses cached initial when form is still missing", () => {
    const createInitial = vi.fn(() => ({ a: 1 }));
    const cached = { a: 1 };

    const result = resolveInitialForm(undefined, cached, createInitial);

    expect(result.value).toBe(cached);
    expect(result.cache).toBe(cached);
    expect(result.shouldHydrate).toBe(true);
    expect(createInitial).not.toHaveBeenCalled();
  });
});
