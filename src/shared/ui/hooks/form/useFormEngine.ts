import { useCallback, useRef } from "react";

type Options<K extends string> = {
  keys: readonly K[];
  onSubmit?: () => void;
};

export function useFormNavigation<K extends string>(
  options: Options<K>
) {
  const { keys, onSubmit } = options;

  const refs = useRef<Partial<Record<K, HTMLInputElement>>>({});

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
    refs.current[key]?.focus();
  }, []);

  const move = useCallback(
    (current: K, direction: 1 | -1) => {
      const index = keys.indexOf(current);
      if (index === -1) return;

      let nextIndex = index + direction;

      while (nextIndex >= 0 && nextIndex < keys.length) {
        const nextKey = keys[nextIndex];
        const el = refs.current[nextKey];

        // skip disabled
        if (el && !el.disabled) {
          el.focus();
          return;
        }

        nextIndex += direction;
      }
    },
    [keys]
  );

  const handleKeyDown = useCallback(
    (key: K) =>
      (e: React.KeyboardEvent<HTMLInputElement>) => {

        if (e.key === "ArrowDown") {
          e.preventDefault();
          move(key, 1);
          return;
        }

        if (e.key === "ArrowUp") {
          e.preventDefault();
          move(key, -1);
          return;
        }

        if (e.key === "Enter") {
          e.preventDefault();

          const index = keys.indexOf(key);
          const isLast = index === keys.length - 1;

          if (isLast && onSubmit) {
            onSubmit();
          } else {
            move(key, 1);
          }
        }
      },
    [keys, move, onSubmit]
  );

  return {
    register,
    focus,
    handleKeyDown,
  };
}
