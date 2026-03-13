// shared/ui/hooks/execution/useExecutionNavigation.ts

import { useCallback, useRef } from "react";

export function useExecutionNavigation(options: {
  stepOrder: readonly number[];
  activeStep?: number;
  onSubmitStep?: (step: number) => void;
}) {

  const { stepOrder, activeStep, onSubmitStep } = options;

  const refs = useRef<Record<number, HTMLInputElement | null>>({});

  /* =========================
     Register
  ========================= */

  const register = useCallback(
    (step: number) => (el: HTMLInputElement | null) => {
      refs.current[step] = el;
    },
    []
  );

  /* =========================
     Focus helpers
  ========================= */

  const focus = useCallback((step?: number) => {
    if (step === undefined) return;

    const el = refs.current[step];
    if (!el || el.disabled) return;

    el.focus();
  }, []);

  const findNext = useCallback(
    (current: number, direction: 1 | -1) => {

      const index = stepOrder.indexOf(current);
      if (index === -1) return;

      let i = index + direction;

      while (i >= 0 && i < stepOrder.length) {
        const step = stepOrder[i];
        const el = refs.current[step];

        if (el && !el.disabled) {
          return step;
        }

        i += direction;
      }

      return undefined;
    },
    [stepOrder]
  );

  /* =========================
     Keyboard navigation
  ========================= */

  const handleKeyDown = useCallback(
    (step: number) =>
      (e: React.KeyboardEvent<HTMLInputElement>) => {

        if (e.key === "Enter") {
          e.preventDefault();
          onSubmitStep?.(step);
          return;
        }

        if (e.key === "ArrowDown") {
          e.preventDefault();
          const next = findNext(step, 1);
          focus(next);
          return;
        }

        if (e.key === "ArrowUp") {
          e.preventDefault();
          const prev = findNext(step, -1);
          focus(prev);
          return;
        }
      },
    [findNext, focus, onSubmitStep]
  );

  /* =========================
     Focus active step
  ========================= */

  const focusActive = useCallback(() => {
    focus(activeStep);
  }, [activeStep, focus]);

  return {
    register,
    focus,
    focusActive,
    handleKeyDown,
  };
}