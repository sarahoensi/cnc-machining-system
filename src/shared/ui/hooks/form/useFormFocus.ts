// shared/ui/hooks/useFormFocus.ts

import { useCallback, useEffect, useMemo, useRef } from "react";
import type { FieldState } from "../../../types/fields";

/**
 * Generic form focus manager.
 *
 * - Registers input refs
 * - Focuses specific fields programmatically
 * - Skips locked fields
 * - Optionally auto-focuses first available field on mount
 */
export function useFormFocus<K extends string>(options: {
  keys: readonly K[];
  fields: Record<K, FieldState>;
  lockedKeys?: readonly K[];
  autoFocusOnMount?: boolean;
}) {
  const { keys, lockedKeys = [], autoFocusOnMount } = options;

  const refs = useRef<Partial<Record<K, HTMLInputElement>>>({});
  const lastFocused = useRef<K | undefined>(undefined);
  const didAutoFocus = useRef(false);

  /**
   * Register input ref for a field key.
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
   * Focus a specific field if it is not locked.
   */
  const focus = useCallback(
    (key?: K) => {
      if (!key) return;
      if (lockedKeys.includes(key)) return;

      const el = refs.current[key];
      if (!el) return;

      el.focus();
      lastFocused.current = key;
    },
    [lockedKeys]
  );

  /**
   * First focusable (non-locked) field in defined order.
   */
  const firstFocusable = useMemo(() => {
    return keys.find((k) => !lockedKeys.includes(k));
  }, [keys, lockedKeys]);

  /**
   * Optional auto-focus on mount.
   */
  useEffect(() => {
    if (!autoFocusOnMount) return;
    if (didAutoFocus.current) return;
    if (!firstFocusable) return;

    didAutoFocus.current = true;
    focus(firstFocusable);
  }, [autoFocusOnMount, firstFocusable, focus]);

  return {
    register,
    focus,
    focusFirst: () => focus(firstFocusable),
    restoreLast: () => focus(lastFocused.current),
  };
}
