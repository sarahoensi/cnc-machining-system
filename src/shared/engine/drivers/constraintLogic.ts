// shared/core/drivers/constraintLogic.ts

import type { FieldState } from "@shared/types/fields";
import { emptyField } from "@shared/types/fields";

/**
 * Evaluates structural constraint sets.
 *
 * Rules:
 * - Only "user" fields drive constraints
 * - Locked fields must always be empty
 * - On conflict, editedKey wins
 * - No locks are applied in solved-mode (handled outside)
 */
export function evaluateConstraints<K extends string>(
  fields: Record<K, FieldState>,
  validSets: readonly (readonly K[])[],
  editedKey: K | null
): {
  fields: Record<K, FieldState>;
  activeSet: readonly K[] | null;
} {

  const keys = Object.keys(fields) as K[];
  let next: Record<K, FieldState> = { ...fields };

  // 🔎 Identify user-driven keys
  const userKeys = keys.filter(
    k => next[k].source === "user"
  );

  /* ============================================================
     1️⃣ No user input → everything unlocked
  ============================================================ */

  if (userKeys.length === 0) {
    for (const k of keys) {
      next[k] = {
        ...next[k],
        locked: false,
        invalid: false,
      };
    }

    return {
      fields: next,
      activeSet: null,
    };
  }

  /* ============================================================
     2️⃣ Find compatible sets
  ============================================================ */

  const possibleSets = validSets.filter(set =>
    userKeys.every(k => set.includes(k))
  );

  /* ============================================================
     3️⃣ Conflict → editedKey wins
  ============================================================ */

  if (possibleSets.length === 0 && editedKey) {

    for (const k of userKeys) {
      if (k !== editedKey) {
        next[k] = emptyField();
      }
    }

    // Re-run evaluation after cleanup
    return evaluateConstraints(next, validSets, editedKey);
  }

  /* ============================================================
     4️⃣ Determine allowed keys (union of possible sets)
  ============================================================ */

  const allowed = new Set<K>();

  for (const set of possibleSets) {
    for (const k of set) {
      allowed.add(k);
    }
  }

  /* ============================================================
     5️⃣ Apply locking rule
     Locked fields must always be empty
  ============================================================ */

  for (const k of keys) {

    if (!allowed.has(k)) {
      // Locked ⇒ empty
      next[k] = emptyField({ locked: true });
    } else {
      // Allowed ⇒ unlocked
      next[k] = {
        ...next[k],
        locked: false,
        invalid: false,
      };
    }
  }

  /* ============================================================
     6️⃣ Determine active set
  ============================================================ */

  const activeSet =
    possibleSets.length === 1
      ? possibleSets[0]
      : null;

  return {
    fields: next,
    activeSet,
  };
}
