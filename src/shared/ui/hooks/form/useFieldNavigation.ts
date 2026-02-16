// shared/ui/hooks/form/useFieldNavigation.ts

import { useCallback, useRef } from "react";

/**
 * Coordinates focus movement between fields (Enter, Shift+Enter, etc.).
 *
 * Pure interaction logic.
 *
 * - No domain knowledge
 * - No validation logic
 * - No driver awareness
 *
 * Works with a fixed ordered list of field keys.
 */
export function useFieldNavigation<K extends string>(keys: readonly K[]) {
  const refs = useRef<Partial<Record<K, HTMLInputElement>>>({});

  /**
   * Register input ref by key.
   */
  const register = useCallback(
    (key: K) => (el: HTMLInputElement | null) => {
      if (el) {
        refs.current[key] = el;
      }
    },
    []
  );

  /**
   * Focus field by key.
   */
  const focus = useCallback((key?: K) => {
    if (!key) return;
    refs.current[key]?.focus();
  }, []);

  /**
   * Focus next field in order.
   */
  const focusNext = useCallback(
    (current: K) => {
      const index = keys.indexOf(current);
      if (index === -1) return;

      const next = keys[index + 1];
      if (next) focus(next);
    },
    [keys, focus]
  );

  /**
   * Focus previous field in order.
   */
  const focusPrev = useCallback(
    (current: K) => {
      const index = keys.indexOf(current);
      if (index === -1) return;

      const prev = keys[index - 1];
      if (prev) focus(prev);
    },
    [keys, focus]
  );

  /**
   * Key handler to attach to inputs.
   * Handles:
   * - Enter → next
   * - Shift+Enter → previous
   */
  const handleKeyDown = useCallback(
    (key: K) =>
      (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === "Enter") {
          e.preventDefault();

          if (e.shiftKey) {
            focusPrev(key);
          } else {
            focusNext(key);
          }
        }
      },
    [focusNext, focusPrev]
  );

  return {
    register,
    focus,
    focusNext,
    focusPrev,
    handleKeyDown,
  };
}
