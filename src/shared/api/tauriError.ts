// shared/api/tauriError.ts

export type TauriFieldError = {
  field: string;
  code: string;
  message: string;
};

export type TauriCommandError = {
  message: string;
  fieldErrors?: TauriFieldError[];
};

function isFieldError(value: unknown): value is TauriFieldError {
  if (!value || typeof value !== "object") return false;
  const v = value as Record<string, unknown>;
  return (
    typeof v.field === "string" &&
    typeof v.code === "string" &&
    typeof v.message === "string"
  );
}

function isTauriCommandError(value: unknown): value is TauriCommandError {
  if (!value || typeof value !== "object") return false;
  const v = value as Record<string, unknown>;
  const hasMessage = typeof v.message === "string";
  const hasFieldErrors =
    Array.isArray(v.fieldErrors) && v.fieldErrors.every(isFieldError);
  return hasMessage || hasFieldErrors;
}

export function getTauriCommandError(e: unknown): TauriCommandError | null {
  if (!e || typeof e !== "object") return null;

  const obj = e as Record<string, unknown>;

  // Case 1: message contains JSON string (common in Tauri)
  if (typeof obj.message === "string") {
    try {
      const parsed: unknown = JSON.parse(obj.message);
      if (isTauriCommandError(parsed)) {
        const parsedObj = parsed as Record<string, unknown>;
        return {
          message:
            typeof parsedObj.message === "string"
              ? parsedObj.message
              : "Unknown error",
          fieldErrors: Array.isArray(parsedObj.fieldErrors)
            ? (parsedObj.fieldErrors as TauriFieldError[])
            : undefined,
        };
      }
    } catch {
      // ignore parse failure
    }
  }

  // Case 2: structured object from Tauri
  if (isTauriCommandError(obj)) {
    return {
      message:
        typeof obj.message === "string" ? obj.message : "Unknown error",
      fieldErrors: Array.isArray(obj.fieldErrors)
        ? (obj.fieldErrors as TauriFieldError[])
        : undefined,
    };
  }

  return null;
}
