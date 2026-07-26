import type { FieldState } from "@shared/form/types";

import type { ThreadCalculationInput } from "../api/types";
import type { ThreadExtras, ThreadKey } from "./threadForm";
import { getPitchApiSelection } from "./threadOptions";

export function parseThread(
  fields: Record<ThreadKey, FieldState>,
  extras: ThreadExtras,
): ThreadCalculationInput | null {
  const size = fields.size.value.trim();
  const pitch = fields.pitch.value.trim();

  if (!size || !pitch) return null;

  const apiSelection = getPitchApiSelection(extras.type, pitch);
  if (!apiSelection) return null;

  return {
    type: apiSelection.type,
    size,
    pitch: apiSelection.pitch,
  };
}
