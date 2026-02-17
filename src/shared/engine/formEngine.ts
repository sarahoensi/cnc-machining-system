// shared/engine/formEngine.ts

import type { FormState } from "@shared/types/forms";
import type { FieldState } from "@shared/types/fields";
import {
  emptyField,
  userField,
  machineField,
} from "@shared/types/fields";

import { applyDriverEngine } from "./drivers";

/**
 * Removes all locks and invalid flags from fields.
 *
 * Used after successful calculation when entering "solved" mode.
 *
 * @param fields - Current field record
 * @returns New field record with all fields unlocked
 */
export function unlockAll<K extends string>(
  fields: Record<K, FieldState>
): Record<K, FieldState> {

  const next = { ...fields };

  for (const key in next) {
    next[key] = {
      ...next[key],
      locked: false,
      invalid: false,
    };
  }

  return next;
}

/**
 * Clears all machine-computed values.
 *
 * Used when:
 * - User edits after solved mode
 * - Before running a new calculation
 *
 * @param fields - Current field record
 * @returns New field record with machine fields reset to empty
 */
export function clearMachineFields<K extends string>(
  fields: Record<K, FieldState>
): Record<K, FieldState> {

  const next = { ...fields };

  for (const key in next) {
    if (next[key].source === "machine") {
      next[key] = emptyField();
    }
  }

  return next;
}

/**
 * Handles user editing of a single field.
 *
 * Lifecycle:
 * 1. If previously solved → clear machine fields
 * 2. Apply user value
 * 3. Run driver engine (constraints + pairs)
 * 4. Return updated editing state
 *
 * Guarantees:
 * - Locked fields are always empty
 * - Driver logic only runs in editing mode
 *
 * @param form - Current form state
 * @param key - Edited field key
 * @param rawValue - Raw string input from user
 * @param validSets - Structural constraint sets
 * @param pairs - Independent driver pairs
 * @returns Updated form state in editing mode
 */
export function handleUserEdit<K extends string>(
  form: FormState<K>,
  key: K,
  rawValue: string,
  validSets: readonly (readonly K[])[],
  pairs: readonly (readonly [K, K])[]
): FormState<K> {

  let nextFields = { ...form.fields };

  // If previously solved, restart editing lifecycle
  if (form.mode === "solved") {
    nextFields = clearMachineFields(nextFields);
  }

  // Apply user value
  nextFields[key] = userField(rawValue);

  // Apply structural + pair driver logic
  const driven = applyDriverEngine(
    nextFields,
    {
      validSets,
      pairs,
      editedKey: key,
      mode: "editing",
    }
  );

  return {
    mode: "editing",
    fields: driven.fields,
  };
}

/**
 * Handles calculation lifecycle when user presses "Calculate".
 *
 * Lifecycle:
 * 1. Parse numeric input
 * 2. Clear previous machine values
 * 3. Run solve algorithm
 * 4. Apply machine-computed values
 * 5. Unlock all fields
 * 6. Enter solved mode
 *
 * @param form - Current form state
 * @param parse - Function converting field strings to numeric input
 * @param solve - Domain solve function
 * @returns Updated form state in solved mode
 */
export function handleCalculate<K extends string>(
  form: FormState<K>,
  parse: (
    fields: Record<K, FieldState>
  ) => Record<K, number> | null,
  solve: (
    input: Record<K, number>
  ) => Partial<Record<K, number>>
): FormState<K> {

  const parsed = parse(form.fields);

  if (!parsed) {
    return form;
  }

  let nextFields = clearMachineFields(form.fields);

  const result = solve(parsed);

  for (const key in result) {
    nextFields[key as K] = machineField(
      String(result[key as K])
    );
  }

  const solvedFields = unlockAll(nextFields);

  return {
    mode: "solved",
    fields: solvedFields,
  };
}
