// shared/ui/hooks/form/useAutoFocus.ts

import { useEffect, useRef } from "react";

/**
 * Automatically focuses an input element when mounted or when
 * the `trigger` value changes.
 *
 * Pure presentation logic.
 *
 * Usage:
 *   const inputRef = useAutoFocus(true);
 *   <input ref={inputRef} />
 */
export function useAutoFocus(
  enabled: boolean = true,
  trigger?: unknown
) {
  const ref = useRef<HTMLInputElement | null>(null);
  const hasFocused = useRef(false);

  useEffect(() => {
    if (!enabled) return;
    if (!ref.current) return;

    // Prevent double-focus in StrictMode
    if (hasFocused.current && trigger === undefined) return;

    ref.current.focus();
    hasFocused.current = true;
  }, [enabled, trigger]);

  return ref;
}
