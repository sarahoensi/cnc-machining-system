// shared/engine/formEngine.ts

import type { FormState } from "@shared/form/types/forms";
import type { FieldState } from "@shared/form/types/fields";
import { emptyField, userField } from "@shared/form/types/fields";
import { applyDriverEngine } from "../constraints";

import { getTauriCommandError } from "@shared/api/tauriError";


/* ============================================================
   Unlock all fields
============================================================ */

export function unlockAll<K extends string>(
  fields: Record<K, FieldState>
): Record<K, FieldState> {

  const next = {} as Record<K, FieldState>;

  for (const key in fields) {
    next[key] = {
      ...fields[key],
      locked: false,
      invalid: false,
      error: undefined,
    };
  }

  return next;
}

/* ============================================================
   Reset all fields
============================================================ */

export function resetForm<K extends string, E>(
  form: FormState<K, E>
): FormState<K, E> {

  const next = {} as Record<K, FieldState>;

  for (const key in form.fields) {
    next[key] = emptyField();
  }

  return {
    status: "editing",
    fields: next,
    extras: form.extras,
  };
}

/* ============================================================
   Clear machine fields
============================================================ */

export function clearMachineFields<K extends string>(
  fields: Record<K, FieldState>
): Record<K, FieldState> {

  const next = {} as Record<K, FieldState>;

  for (const key in fields) {
    next[key] =
      fields[key].source === "machine"
        ? emptyField()
        : fields[key];
  }

  return next;
}

/* ============================================================
   USER EDIT
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

  // If editing after solved → remove machine values
  if (form.status === "solved") {
    nextFields = clearMachineFields(nextFields);
  }

  const updatedFields = {
    ...nextFields,
    [key]: { ...userField(rawValue), error: undefined, invalid: false },
  };

  const driven = applyDriverEngine(updatedFields, {
    validSets,
    pairs,
    editedKey: key,
    mode: "editing",
  });

  return {
    status: "editing",
    fields: driven.fields,
    extras: form.extras,
  };
}

/* ============================================================
   HandleModeChange
============================================================ */

export function handleModeChange<
  K extends string,
  E extends { mode: unknown }
>(
  form: FormState<K, E>,
  newExtras: E
): FormState<K, E> {

  const modeChanged =
    form.extras.mode !== newExtras.mode;

  if (!modeChanged) {
    return form;
  }

  return {
    status: "editing",
    fields: clearMachineFields(form.fields),
    extras: newExtras,
  };
}

/* ============================================================
   ASYNC CALCULATE
============================================================ */

export async function handleCalculateAsync<
  K extends string,
  E
>(
  form: FormState<K, E>,
  parse: (
    fields: Record<K, FieldState>,
    extras: E
  ) => Partial<Record<K, number>> | null,
  solve: (
    input: Partial<Record<K, number>>,
    extras: E
  ) => Promise<Partial<Record<K, number>>>,
): Promise<FormState<K, E>> {

  // 1️⃣ Parse input
  const parsed = parse(form.fields, form.extras);

  if (!parsed) {
    return form; // Ingenting å beregne
  }

  // 2️⃣ Clear gamle machine-verdier
  const cleanedFields = clearMachineFields(form.fields);

  try {

    // 3️⃣ Kjør solver
    const result = await solve(parsed, form.extras);

    const nextFields: Record<K, FieldState> = {
      ...cleanedFields,
    };

    // 4️⃣ Sett machine-verdier riktig formatert
    for (const key in result) {

      const k = key as K;
      const value = result[k];

      if (value === undefined || value === null) continue;

      const wasUser = cleanedFields[k]?.source === "user";

      nextFields[k] = {
        ...cleanedFields[k],
        value: String(value), 
        machineValue: value, 
        source: wasUser ? "user" : "machine",
        locked: false,
        invalid: false,
        error: undefined,
      };
    }

    return {
      status: "solved",
      fields: unlockAll(nextFields),
      extras: form.extras,
    };

} catch (error) {

  console.error(error);

  const te = getTauriCommandError(error);

  const nextFields = { ...cleanedFields };

  if (te?.fieldErrors) {

    for (const err of te.fieldErrors) {
      const k = err.field as K;

      if (!nextFields[k]) continue;

      nextFields[k] = {
        ...nextFields[k],
        invalid: true,
        error: err.message,
      };
    }

  }

  return {
    status: "editing",
    fields: nextFields,
    extras: form.extras,
  };
}
}

/* ============================================================
   ASYNC GENERATE
============================================================ */
export async function handleGenerateAsync<
  K extends string,
  E,
  I,
  X
>(
  form: FormState<K, E>,
  parse: (
    fields: Record<K, FieldState>,
    extras: E
  ) => I | null,
  execute: (
    input: I,
    extras: E
  ) => Promise<X>
): Promise<{
  form: FormState<K, E>;
  execution?: X;
}> {

  const parsed = parse(form.fields, form.extras);

  if (!parsed) {
    return { form };
  }

  try {

    const execution = await execute(parsed, form.extras);

    return {
      form,
      execution,
    };

  } catch (error) {

    console.error(error);

    const te = getTauriCommandError(error);

    const nextFields = { ...form.fields };

    if (te?.fieldErrors) {

      for (const err of te.fieldErrors) {

        const k = err.field as K;

        if (!nextFields[k]) continue;

        nextFields[k] = {
          ...nextFields[k],
          invalid: true,
          error: err.message,
        };
      }
    }

    return {
      form: {
        ...form,
        fields: nextFields,
      }
    };
  }
}