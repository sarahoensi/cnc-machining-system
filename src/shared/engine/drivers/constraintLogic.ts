// shared/core/drivers/constraintLogic.ts

import type { FieldState } from "@shared/types/fields";
import { emptyField } from "@shared/types/fields";

/**
 * Resolves which constraint set is active based on user-filled fields.
 */
export function resolveActiveSet<
  F extends Record<string, FieldState>,
  K extends keyof F & string
>(
  fields: F,
  validSets: readonly (readonly K[])[]
): readonly K[] | null {

  const filled = (Object.keys(fields) as K[])
    .filter(k => fields[k].source === "user");

  if (filled.length === 0) return null;

  const match = validSets.find(set =>
    filled.every(f => set.includes(f))
  );

  return match ?? null;
}

/**
 * Returns keys that are not part of the active constraint set.
 */
export function resolveLockedKeys<
  F extends Record<string, FieldState>,
  K extends keyof F & string
>(
  fields: F,
  validSets: readonly (readonly K[])[]
): K[] {

  const active = resolveActiveSet(fields, validSets);
  if (!active) return [];

  return (Object.keys(fields) as K[])
    .filter(k => !active.includes(k));
}

/**
 * Clears conflicting user inputs when switching constraint driver.
 */
export function applyConstraintSwitch<
  F extends Record<string, FieldState>,
  K extends keyof F & string
>(
  fields: F,
  editedKey: K,
  validSets: readonly (readonly K[])[]
): F {

  const next = { ...fields };

  const possibleSets = validSets.filter(set =>
    set.includes(editedKey)
  );

  if (possibleSets.length === 0) return fields;

  for (const set of possibleSets) {
    for (const key of set) {
      if (
        key !== editedKey &&
        fields[key].source === "user"
      ) {
        next[key] = emptyField() as F[K];
      }
    }
  }

  return next;
}
