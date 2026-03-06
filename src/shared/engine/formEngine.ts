// shared/engine/formEngine.ts

import type { FormState } from "@shared/types/forms";
import type { FieldState } from "@shared/types/fields";
import { emptyField, userField } from "@shared/types/fields";
import { applyDriverEngine } from "./drivers";

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

      if (value === undefined) continue;

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

  console.log("RAW ERROR:", error);

  const te = getTauriCommandError(error);
  console.log("PARSED TAURI ERROR:", te);

  

  // Start med "cleaned", så du ikke viser stale machine values
  const nextFields: Record<K, FieldState> = { ...cleanedFields };

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