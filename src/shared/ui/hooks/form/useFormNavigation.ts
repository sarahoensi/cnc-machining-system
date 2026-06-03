import { useCallback, useEffect, useRef } from "react";

type FormNavigationControl = HTMLInputElement | HTMLButtonElement;

function isReadOnlyControl(el: FormNavigationControl) {
  return el instanceof HTMLInputElement && el.readOnly;
}

export function useFormNavigation<K extends string>(options: {
  keys: readonly K[];
  autoFocusOnMount?: boolean;
  onSubmit?: () => void;
  activePath?: string;
}) {
  const { keys, autoFocusOnMount = false, onSubmit, activePath } = options;

  const refs = useRef<Partial<Record<K, FormNavigationControl>>>({});
  const submitActionRef = useRef<HTMLButtonElement | null>(null);
  const lastFocused = useRef<K | undefined>(undefined);
  const didAutoFocus = useRef(false);
  const containerRef = useRef<HTMLDivElement | null>(null);

  const register = useCallback(
    (key: K) => (el: FormNavigationControl | null) => {
      if (el) refs.current[key] = el;
      else delete refs.current[key];
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

  const registerSubmitAction = useCallback((el: HTMLButtonElement | null) => {
    submitActionRef.current = el;
  }, []);

  const focusSubmitAction = useCallback(() => {
    const el = submitActionRef.current;
    if (!el || el.disabled || el.tabIndex === -1) return false;

    el.focus();
    return true;
  }, []);

  const focusAfterRender = useCallback(
    (key?: K) => {
      if (!key) return;
      requestAnimationFrame(() => {
        focus(key);
      });
    },
    [focus]
  );

  const focusFirst = useCallback(() => {
    for (const key of keys) {
      const el = refs.current[key];
      if (el && !el.disabled && el.tabIndex !== -1 && !isReadOnlyControl(el)) {
        focus(key);
        return true;
      }
    }
    return false;
  }, [focus, keys]);

  const focusFirstMatching = useCallback(
    (match: (key: K, el: FormNavigationControl) => boolean) => {
      for (const key of keys) {
        const el = refs.current[key];
        if (!el) continue;
        if (el.disabled || isReadOnlyControl(el) || el.tabIndex === -1) continue;
        if (el.offsetParent === null) continue;
        if (!match(key, el)) continue;
        focus(key);
        return key;
      }
      return undefined;
    },
    [focus, keys]
  );

  const focusFirstMatchingAfterRender = useCallback(
    (match: (key: K, el: FormNavigationControl) => boolean) => {
      requestAnimationFrame(() => {
        focusFirstMatching(match);
      });
    },
    [focusFirstMatching]
  );

  const focusFirstInOrder = useCallback(
    (order: readonly K[], match?: (key: K, el: FormNavigationControl) => boolean) => {
      for (const key of order) {
        const el = refs.current[key];
        if (!el) continue;
        if (el.disabled || isReadOnlyControl(el) || el.tabIndex === -1) continue;
        if (el.offsetParent === null) continue;
        if (match && !match(key, el)) continue;
        focus(key);
        return key;
      }
      return undefined;
    },
    [focus]
  );

  const focusFirstInOrderAfterRender = useCallback(
    (order: readonly K[], match?: (key: K, el: FormNavigationControl) => boolean) => {
      requestAnimationFrame(() => {
        focusFirstInOrder(order, match);
      });
    },
    [focusFirstInOrder]
  );

  const focusFirstAfterRender = useCallback(() => {
    requestAnimationFrame(() => {
      focusFirst();
    });
  }, [focusFirst]);

  const focusFirstInvalid = useCallback(
    (hasError: (key: K) => boolean) => {
      for (const key of keys) {
        const el = refs.current[key];
        if (!el) continue;
        if (el.disabled || isReadOnlyControl(el) || el.tabIndex === -1) continue;
        if (!hasError(key)) continue;
        focus(key);
        return key;
      }
      return undefined;
    },
    [focus, keys]
  );

  const focusFirstInvalidAfterRender = useCallback(
    (hasError: (key: K) => boolean) => {
      requestAnimationFrame(() => {
        focusFirstInvalid(hasError);
      });
    },
    [focusFirstInvalid]
  );

  const hasFocusWithin = useCallback(() => {
    const active = document.activeElement;
    if (!active) return false;
    const root = containerRef.current;
    if (root && root.contains(active)) return true;
    return Object.values(refs.current).some((el) => el === active);
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

  const handleKeyDown = useCallback(
    (key: K) => (e: React.KeyboardEvent<FormNavigationControl>) => {
      if (e.key === "Enter") {
        e.preventDefault();

        if (e.shiftKey) {
          const prev = findNext(key, -1);
          focus(prev);
          return;
        }

        if (isLastFocusable(key)) {
          onSubmit?.();
          return;
        }

        const next = findNext(key, 1);
        focus(next);
        return;
      }

      if (e.key === "ArrowDown") {
        e.preventDefault();
        const next = findNext(key, 1);
        if (next) {
          focus(next);
          return;
        }
        focusSubmitAction();
        return;
      }

      if (e.key === "ArrowUp") {
        e.preventDefault();
        const prev = findNext(key, -1);
        focus(prev);
      }
    },
    [findNext, focus, focusSubmitAction, isLastFocusable, onSubmit]
  );

  const handleSubmitActionKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLButtonElement>) => {
      if (e.key !== "ArrowUp") return;

      e.preventDefault();
      const lastKey = keys.findLast((key) => {
        const el = refs.current[key];
        return Boolean(el && !el.disabled && el.tabIndex !== -1);
      });
      focus(lastKey);
    },
    [focus, keys]
  );

  useEffect(() => {
    if (!autoFocusOnMount) return;
    if (didAutoFocus.current) return;

    if (focusFirst()) {
      didAutoFocus.current = true;
    }
  }, [autoFocusOnMount, focusFirst]);

  useEffect(() => {
    if (!activePath) return;

    const onActiveNavClick = (event: Event) => {
      const custom = event as CustomEvent<{ path?: string }>;
      if (custom.detail?.path !== activePath) return;
      if (hasFocusWithin()) return;
      focusFirstAfterRender();
    };

    window.addEventListener("app:active-nav-click", onActiveNavClick);
    return () => {
      window.removeEventListener("app:active-nav-click", onActiveNavClick);
    };
  }, [activePath, focusFirstAfterRender, hasFocusWithin]);

  return {
    register,
    registerSubmitAction,
    focus,
    focusAfterRender,
    focusFirst,
    focusFirstMatching,
    focusFirstMatchingAfterRender,
    focusFirstInOrder,
    focusFirstInOrderAfterRender,
    focusFirstAfterRender,
    focusFirstInvalid,
    focusFirstInvalidAfterRender,
    hasFocusWithin,
    containerRef,
    handleKeyDown,
    handleSubmitActionKeyDown,
    restoreLast: () => focus(lastFocused.current),
  };
}
