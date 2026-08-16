/**
 * @vitest-environment jsdom
 */

import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { useAutoCalculate } from "./useAutoCalculate";

describe("useAutoCalculate", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("does not run when disabled", () => {
    const calculate = vi.fn(async (form: string) => `${form}-result`);
    const onResult = vi.fn();

    renderHook(() =>
      useAutoCalculate({
        enabled: false,
        signature: "input",
        form: "input",
        calculate,
        onResult,
      }),
    );

    act(() => {
      vi.advanceTimersByTime(250);
    });

    expect(calculate).not.toHaveBeenCalled();
    expect(onResult).not.toHaveBeenCalled();
  });

  it("debounces calculation and ignores stale results", async () => {
    const calculate = vi.fn(
      (form: string) =>
        new Promise<string>((resolve) => {
          window.setTimeout(() => resolve(`${form}-result`), form === "a" ? 20 : 1);
        }),
    );
    const onResult = vi.fn();

    const { rerender } = renderHook(
      ({ signature, form }) =>
        useAutoCalculate({
          enabled: true,
          signature,
          form,
          calculate,
          onResult,
          debounceMs: 10,
        }),
      {
        initialProps: {
          signature: "a",
          form: "a",
        },
      },
    );

    act(() => {
      vi.advanceTimersByTime(10);
    });

    rerender({
      signature: "b",
      form: "b",
    });

    act(() => {
      vi.advanceTimersByTime(10);
      vi.advanceTimersByTime(20);
    });

    await act(async () => {
      await Promise.resolve();
    });

    expect(calculate).toHaveBeenCalledWith("a");
    expect(calculate).toHaveBeenCalledWith("b");
    expect(onResult).toHaveBeenCalledTimes(1);
    expect(onResult).toHaveBeenCalledWith("b-result");
  });
});
