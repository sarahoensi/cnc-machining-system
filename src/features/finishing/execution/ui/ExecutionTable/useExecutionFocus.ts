// features/finishing/execution/ui/ExecutionTable/useExecutionFocus.ts

import { useCallback, useEffect } from "react";

export function useExecutionFocus<K extends number>(options: {
  inputRefs: React.RefObject<Record<K, HTMLInputElement | null>>;
  activeIndex: K | null;
  editingIndex: K | null;
}) {
  const { inputRefs, activeIndex, editingIndex } = options;

  const focus = useCallback(
    (index: K | null) => {
      if (index == null) return;

      requestAnimationFrame(() => {
        const el = inputRefs.current[index];
        if (el && !el.disabled && el.tabIndex !== -1) {
          el.focus();
        }
      });
    },
    [inputRefs],
  );

  useEffect(() => {
    if (editingIndex !== null) {
      focus(editingIndex);
      return;
    }

    focus(activeIndex);
  }, [editingIndex, activeIndex, focus]);
}
