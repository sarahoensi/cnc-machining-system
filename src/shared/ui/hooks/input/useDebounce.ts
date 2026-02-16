// shared/ui/hooks/input/useDebounce.ts

import { useEffect, useState } from "react";

/**
 * Debounces a changing value over time.
 *
 * Returns a delayed version of the input value.
 * Useful to prevent excessive recalculation or backend calls.
 *
 * Example:
 * const debounced = useDebounce(value, 300);
 */
export function useDebounce<T>(
  value: T,
  delay: number
): T {
  const [debouncedValue, setDebouncedValue] = useState(value);

  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(timer);
    };
  }, [value, delay]);

  return debouncedValue;
}
