// shared/engine/formEngine.ts

import type { FormState } from "@shared/form/types/forms";
import type { FieldState } from "@shared/form/types/fields";
import { emptyField, userField } from "@shared/form/types/fields";
import { applyDriverEngine } from "../constraints";

import { getTauriCommandError } from "@shared/api/tauriError";

import {
  parseDecimalInput
} from "@shared/parsing/decimalParser";


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

  const prev = form.fields[key];
  const { normalized } = parseDecimalInput(rawValue);

if (!didUserEdit(prev, normalized)) {
  return form;
}

  let nextFields = form.fields;

  // If editing after solved → remove machine values
  if (form.status === "solved") {
    nextFields = clearMachineFields(nextFields);
  }

  const updatedFields = {
    ...nextFields,
    [key]: {
      ...userField(normalized),
      error: undefined,
      invalid: false,
    },
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

function didUserEdit(
  prev: FieldState,
  nextValue: string
): boolean {

  if (prev.source !== "user") {
    return true;
  }

  const prevNorm = parseDecimalInput(prev.value ?? "").normalized;

  return prevNorm !== nextValue;
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
   Apply field errors
============================================================ */

function applyFieldErrors<K extends string>(
  fields: Record<K, FieldState>,
  error: unknown
): Record<K, FieldState> {

  const te = getTauriCommandError(error);

  if (!te?.fieldErrors) {
    return fields;
  }

  const next = { ...fields };

  for (const err of te.fieldErrors) {

    const k = err.field as K;

    if (!next[k]) continue;

    next[k] = {
      ...next[k],
      invalid: true,
      error: err.message,
    };
  }

  return next;
}

/* ============================================================
   Clean field errors
============================================================ */


function clearFieldErrors<K extends string>(
  fields: Record<K, FieldState>
): Record<K, FieldState> {

  const next = {} as Record<K, FieldState>;

  for (const key in fields) {
    next[key] = {
      ...fields[key],
      invalid: false,
      error: undefined,
    };
  }

  return next;
}

/* ============================================================
   Field Normalization
============================================================ */

export function applyFieldNormalization<K extends string, E>(
  form: FormState<K, E>,
  key: K,
  value: string
): FormState<K, E> {

  const prev = form.fields[key];

  if (prev.value === value) {
    return form;
  }

  return {
    ...form,
    fields: {
      ...form.fields,
      [key]: {
        ...prev,
        value,
      },
    },
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

      if (wasUser) {
        nextFields[k] = {
          ...cleanedFields[k],
          machineValue: value,
          invalid: false,
          error: undefined,
        };
      } else {
        nextFields[k] = {
          ...cleanedFields[k],
          value: String(value),
          machineValue: value,
          source: "machine",
          invalid: false,
          error: undefined,
        };
      }
    }

    return {
      status: "solved",
      fields: unlockAll(nextFields),
      extras: form.extras,
    };

  } catch (error) {

    console.error(error);

    const nextFields = applyFieldErrors(
      cleanedFields,
      error
    );

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

  const cleanedFields = clearFieldErrors(form.fields);

  try {

    const execution = await execute(parsed, form.extras);

    return {
      form: {
        ...form,
        fields: cleanedFields,
      },
      execution,
    };
  } catch (error) {

    console.error(error);

    const nextFields = applyFieldErrors(
      form.fields,
      error
    );

    return {
      form: {
        ...form,
        fields: nextFields,
      }
    };
  }
}