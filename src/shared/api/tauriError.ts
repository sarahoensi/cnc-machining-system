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

export function getTauriCommandError(e: unknown): TauriCommandError | null {

  if (!e || typeof e !== "object") return null;

  const anyE = e as any;

  // Case 1: Tauri sender allerede structured object
  if (anyE.fieldErrors) {
    return {
      message: anyE.message ?? "Unknown error",
      fieldErrors: anyE.fieldErrors,
    };
  }

  // Case 2: message inneholder JSON string (vanlig i Tauri)
  if (typeof anyE.message === "string") {
    try {

      const parsed = JSON.parse(anyE.message);

      if (parsed?.fieldErrors) {
        return parsed;
      }

    } catch {
      // ignore parse failure
    }
  }

  return null;
}