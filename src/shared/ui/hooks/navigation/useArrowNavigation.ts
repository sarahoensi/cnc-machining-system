import { useCallback } from "react";

/**
 * Enables keyboard arrow navigation between form fields.
 *
 * Pure UI behavior.
 * - ArrowDown → next enabled field
 * - ArrowUp   → previous enabled field
 *
 * Skips locked fields.
 */
export function useArrowNavigation<K extends string>(options: {
  keys: readonly K[];
  lockedKeys?: readonly K[];
  focus: (key: K) => void;
}) {
  const { keys, lockedKeys = [], focus } = options;

  const isLocked = (key: K) => lockedKeys.includes(key);

  const getNextKey = (current: K, direction: 1 | -1): K | null => {
    const index = keys.indexOf(current);
    if (index === -1) return null;

    let nextIndex = index + direction;

    while (nextIndex >= 0 && nextIndex < keys.length) {
      const candidate = keys[nextIndex];
      if (!isLocked(candidate)) {
        return candidate;
      }
      nextIndex += direction;
    }

    return null;
  };

  /**
   * Returns an onKeyDown handler for a specific field.
   */
  const createKeyHandler =
    (currentKey: K) =>
    (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        const next = getNextKey(currentKey, 1);
        if (next) focus(next);
      }

      if (e.key === "ArrowUp") {
        e.preventDefault();
        const prev = getNextKey(currentKey, -1);
        if (prev) focus(prev);
      }
    };

  return {
    createKeyHandler,
  };
}
