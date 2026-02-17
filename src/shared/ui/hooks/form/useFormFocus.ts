import { useCallback, useEffect, useMemo, useRef } from "react";

export function useFormFocus<K extends string>(options: {
  keys: readonly K[];
  autoFocusOnMount?: boolean;
}) {
  const { keys, autoFocusOnMount } = options;

  const refs = useRef<Partial<Record<K, HTMLInputElement>>>({});
  const lastFocused = useRef<K | undefined>(undefined);
  const didAutoFocus = useRef(false);

  const register = useCallback(
    (key: K) => (el: HTMLInputElement | null) => {
      if (el) {
        refs.current[key] = el;
      }
    },
    []
  );

  const focus = useCallback((key?: K) => {
    if (!key) return;

    const el = refs.current[key];
    if (!el || el.disabled) return;

    el.focus();
    lastFocused.current = key;
  }, []);

  const firstFocusable = useMemo(() => {
    for (const key of keys) {
      const el = refs.current[key];
      if (el && !el.disabled) return key;
    }
    return undefined;
  }, [keys]);

  useEffect(() => {
    if (!autoFocusOnMount) return;
    if (didAutoFocus.current) return;

    const key = firstFocusable;
    if (!key) return;

    didAutoFocus.current = true;
    focus(key);
  }, [autoFocusOnMount, firstFocusable, focus]);

  return {
    register,
    focus,
    focusFirst: () => focus(firstFocusable),
    restoreLast: () => focus(lastFocused.current),
  };
}
