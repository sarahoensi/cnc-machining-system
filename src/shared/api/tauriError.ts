// shared/api/tauriError.ts

export type TauriCommandError = {
  message: string;
  field_errors?: Record<string, string>;
};

export function getTauriCommandError(e: unknown): TauriCommandError | null {
  // Avhenger av hvordan tauriInvoke kaster errors.
  // Vanlig: e er object med `message` som enten er tekst eller JSON.
  if (!e || typeof e !== "object") return null;

  const anyE = e as any;

  // Case 1: already structured
  if (typeof anyE.message === "string" && anyE.field_errors && typeof anyE.field_errors === "object") {
    return { message: anyE.message, field_errors: anyE.field_errors };
  }

  // Case 2: message is JSON string (ofte i Tauri)
  if (typeof anyE.message === "string") {
    try {
      const parsed = JSON.parse(anyE.message);
      if (parsed && typeof parsed.message === "string") {
        return parsed;
      }
    } catch {
      // ignore
    }
  }

  return null;
}