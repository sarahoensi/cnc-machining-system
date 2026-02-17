// shared/engine/formEngine.ts

/**
 * FormEngine
 *
 * Pure state transition layer for form lifecycle.
 *
 * Responsibilities:
 * - User editing orchestration
 * - Calculation lifecycle
 * - Execution lifecycle
 *
 * Does NOT:
 * - Perform validation
 * - Perform parsing
 * - Perform domain solving
 * - Manage UI state
 */


import type { FormState } from "@shared/types/forms";
import type { FieldState } from "@shared/types/fields";
import {
  emptyField,
  userField,
  machineField,
} from "@shared/types/fields";

import { applyDriverEngine } from "./drivers";

/* ============================================================
   Utility: Unlock all fields
   Used after successful calculation when entering solved mode
============================================================ */

export function unlockAll<K extends string>(
  fields: Record<K, FieldState>
): Record<K, FieldState> {
  const next: Record<K, FieldState> = {} as Record<K, FieldState>;

  for (const key in fields) {
    next[key] = {
      ...fields[key],
      locked: false,
      invalid: false,
    };
  }

  return next;
}

/* ============================================================
   Utility: Clear all machine-computed values
   Used when:
   - User edits after solved mode
   - Before new calculation
============================================================ */

export function clearMachineFields<K extends string>(
  fields: Record<K, FieldState>
): Record<K, FieldState> {
  const next: Record<K, FieldState> = {} as Record<K, FieldState>;

  for (const key in fields) {
    if (fields[key].source === "machine") {
      next[key] = emptyField();
    } else {
      next[key] = fields[key];
    }
  }

  return next;
}

/* ============================================================
   USER EDIT HANDLER
============================================================ */

export function handleUserEdit<
  K extends string,
  E
>(
  form: FormState<K, E>,
  key: K,
  rawValue: string,
  validSets: readonly (readonly K[])[],
  pairs: readonly (readonly [K, K])[]
): FormState<K, E> {

  let nextFields = form.fields;

  // If previously solved → restart editing lifecycle
  if (form.status === "solved") {
    nextFields = clearMachineFields(nextFields);
  }

  // Apply user value
  const updatedFields = {
    ...nextFields,
    [key]: userField(rawValue),
  };

  // Apply constraint + pair driver logic
  const driven = applyDriverEngine(
    updatedFields,
    {
      validSets,
      pairs,
      editedKey: key,
      mode: "editing",
    }
  );

  return {
    status: "editing",
    fields: driven.fields,
    extras: form.extras,
  };
}

/* ============================================================
   CALCULATE HANDLER
============================================================ */

export function handleCalculate<
  K extends string,
  E
>(
  form: FormState<K, E>,
  parse: (
    fields: Record<K, FieldState>,
    extras: E
  ) => Record<K, number> | null,
  solve: (
    input: Record<K, number>,
    extras: E
  ) => Partial<Record<K, number>>
): FormState<K, E> {

  // 1️⃣ Parse
  const parsed = parse(form.fields, form.extras);

  if (!parsed) {
    return form;
  }

  // 2️⃣ Clear old machine values
  let nextFields = clearMachineFields(form.fields);

  // 3️⃣ Solve
  const result = solve(parsed, form.extras);

  // 4️⃣ Apply machine values
  const solvedFields: Record<K, FieldState> = {
    ...nextFields,
  };

  for (const key in result) {
    solvedFields[key as K] = machineField(
      String(result[key as K])
    );
  }

  // 5️⃣ Unlock everything
  const unlocked = unlockAll(solvedFields);

  return {
    status: "solved",
    fields: unlocked,
    extras: form.extras,
  };
}

export function startExecution<K extends string, E>(
  form: FormState<K, E>
): FormState<K, E> {
  return {
    ...form,
    status: "executing",
  };
}

export function stopExecution<K extends string, E>(
  form: FormState<K, E>
): FormState<K, E> {
  return {
    ...form,
    status: "editing",
  };
}
