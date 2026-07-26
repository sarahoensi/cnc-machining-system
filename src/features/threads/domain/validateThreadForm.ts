import type { FieldState } from "@shared/form";

import { getThreadPitch, getThreadSize } from "./threadOptions";
import type { ThreadExtras, ThreadKey } from "./threadForm";

export function validateThreadForm(
  fields: Record<ThreadKey, FieldState>,
  extras: ThreadExtras,
) {
  const errors: string[] = [];

  if (!getThreadSize(extras.options, extras.type, fields.size.value)) {
    errors.push("Thread size is required");
  }

  if (!getThreadPitch(extras.options, extras.type, fields.size.value, fields.pitch.value)) {
    errors.push("Pitch is required");
  }

  return errors.length > 0 ? errors : null;
}
