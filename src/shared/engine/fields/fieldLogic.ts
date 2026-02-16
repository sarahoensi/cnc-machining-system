// shared/utils/fieldLogic.ts

import type { FieldState } from "../../types/fields";
import { emptyField, machineField, userField } from "../../types/fields";

/**
 * Machine values may overwrite anything except real user input.
 */
export function canOverwriteWithMachine(
  field: FieldState
): boolean {
  return !(
    field.source === "user" &&
    field.value !== ""
  );
}

/**
 * Apply machine-computed value safely.
 */
export function applyMachineValue(
  field: FieldState,
  value: string
): FieldState {

  if (!canOverwriteWithMachine(field)) {
    return field;
  }

  return machineField(value);
}

/**
 * Apply user edit.
 * If value becomes empty → source becomes "empty".
 */
export function applyUserValue(
  value: string
): FieldState {
  return userField(value);
}

/**
 * Clear all machine-computed fields.
 */
export function clearMachineFields<
  F extends Record<string, FieldState>
>(fields: F): F {

  let next: F | null = null;

  for (const key of Object.keys(fields) as (keyof F)[]) {

    if (fields[key].source === "machine") {
      if (!next) next = { ...fields };
      next[key] = emptyField() as F[keyof F];
    }
  }

  return next ?? fields;
}
