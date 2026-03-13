// shared/ui/hooks/useFieldErrors.ts

import { useState } from "react";

/**
 * Generic hook for field-level validation errors.
 */
export function useFieldErrors<K extends string>() {
  const [errors, setErrors] = useState<Partial<Record<K, string>>>({});

  /** Replace all field errors */
  function setFieldErrors(next: Partial<Record<K, string>>) {
    setErrors(next);
  }

  /** Clear a single field error */
  function clearFieldError(key: K) {
    setErrors(prev => {
      if (!prev[key]) return prev;
      const copy = { ...prev };
      delete copy[key];
      return copy;
    });
  }

  /** Clear all field errors */
  function clearAllFieldErrors() {
    setErrors({});
  }

  return {
    fieldErrors: errors,
    setFieldErrors,
    clearFieldError,
    clearAllFieldErrors,
  };
}
