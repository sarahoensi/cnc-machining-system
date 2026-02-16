// shared/ui/hooks/navigation/useEnterToSubmit.ts

import { useCallback } from "react";

/**
 * Handles Enter-key submission behavior for forms.
 *
 * - If current field is NOT last → focus next
 * - If current field IS last → trigger submit
 *
 * Pure UI-level orchestration.
 */
export function useEnterToSubmit<K extends string>(options: {
  keys: readonly K[];
  onSubmit: () => void;
  focusNext: (key: K) => void;
}) {
  const { keys, onSubmit, focusNext } = options;

  /**
   * Returns a keydown handler bound to a specific field.
   */
  const handleKeyDown = useCallback(
    (currentKey: K) =>
      (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key !== "Enter") return;

        e.preventDefault();

        const index = keys.indexOf(currentKey);
        const isLast = index === keys.length - 1;

        if (isLast) {
          onSubmit();
        } else {
          focusNext(currentKey);
        }
      },
    [keys, onSubmit, focusNext]
  );

  return {
    handleKeyDown,
  };
}
