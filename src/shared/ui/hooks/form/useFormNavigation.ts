// shared/ui/hooks/form/useFormNavigation.ts

import { useCallback, useEffect, useRef } from "react";

export function useFormNavigation<K extends string>(options: {
  keys: readonly K[];
  autoFocusOnMount?: boolean;
  onSubmit?: () => void; // 🔥 brukes når siste felt
}) {
  const { keys, autoFocusOnMount = false, onSubmit } = options;

  const refs = useRef<Partial<Record<K, HTMLInputElement>>>({});
  const lastFocused = useRef<K | undefined>(undefined);
  const didAutoFocus = useRef(false);

  /* =========================
     Register
  ========================= */

  const register = useCallback(
    (key: K) => (el: HTMLInputElement | null) => {
      if (el) refs.current[key] = el;
    },
    []
  );

  /* =========================
     Focus helpers
  ========================= */

  const focus = useCallback((key?: K) => {
    if (!key) return;

    const el = refs.current[key];
    if (!el || el.disabled) return;

    el.focus();
    lastFocused.current = key;
  }, []);

  const findNext = useCallback(
    (current: K, direction: 1 | -1) => {
      const index = keys.indexOf(current);
      if (index === -1) return undefined;

      let i = index + direction;

      while (i >= 0 && i < keys.length) {
        const key = keys[i];
        const el = refs.current[key];
        if (el && !el.disabled) return key;
        i += direction;
      }

      return undefined;
    },
    [keys]
  );

  const isLastFocusable = useCallback(
    (key: K) => {
      const next = findNext(key, 1);
      return !next;
    },
    [findNext]
  );

  /* =========================
     Keyboard navigation
  ========================= */

  const handleKeyDown = useCallback(
    (key: K) =>
      (e: React.KeyboardEvent<HTMLInputElement>) => {

        // ENTER
        if (e.key === "Enter") {
          e.preventDefault();

          if (e.shiftKey) {
            const prev = findNext(key, -1);
            focus(prev);
            return;
          }

          if (isLastFocusable(key)) {
            onSubmit?.(); // 🔥 Enter = Calculate
            return;
          }

          const next = findNext(key, 1);
          focus(next);
          return;
        }

        // Arrow Down
        if (e.key === "ArrowDown") {
          e.preventDefault();
          const next = findNext(key, 1);
          focus(next);
          return;
        }

        // Arrow Up
        if (e.key === "ArrowUp") {
          e.preventDefault();
          const prev = findNext(key, -1);
          focus(prev);
          return;
        }
      },
    [findNext, focus, isLastFocusable, onSubmit]
  );

  /* =========================
     Auto focus on mount
  ========================= */

  useEffect(() => {
    if (!autoFocusOnMount) return;
    if (didAutoFocus.current) return;

    for (const key of keys) {
      const el = refs.current[key];
      if (el && !el.disabled) {
        didAutoFocus.current = true;
        focus(key);
        break;
      }
    }
  }, [autoFocusOnMount, keys, focus]);

  return {
    register,
    focus,
    handleKeyDown,
    restoreLast: () => focus(lastFocused.current),
  };
}