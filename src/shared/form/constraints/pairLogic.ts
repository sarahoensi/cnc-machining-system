// shared/core/drivers/pairLogic.ts

import type { FieldState } from "@shared/form/types/fields";
import { emptyField } from "@shared/form/types/fields";

/**
 * Applies pair-based driver logic.
 *
 * Rules:
 * - Only active in "editing" mode
 * - If one field in a pair is user-driven,
 *   the other becomes locked and empty
 * - If both become user-driven,
 *   editedKey wins and the other is cleared
 * - Locked fields must always be empty
 */
export function applyPairLogic<K extends string>(
  fields: Record<K, FieldState>,
  pairs: readonly (readonly [K, K])[],
  editedKey: K | null,
  mode: "editing" | "solved"
): Record<K, FieldState> {

  // 🔵 No locking in solved mode
  if (mode === "solved") {
    return fields;
  }

  let next: Record<K, FieldState> = { ...fields };

  for (const [a, b] of pairs) {

    const aIsUser = next[a].source === "user";
    const bIsUser = next[b].source === "user";

    /* ============================================================
       1️⃣ Conflict: both are user
       editedKey wins
    ============================================================ */

    if (aIsUser && bIsUser && editedKey) {

      const loser = editedKey === a ? b : a;

      next[loser] = emptyField({ locked: true });

      continue;
    }

    /* ============================================================
       2️⃣ Normal driver locking
    ============================================================ */

    if (aIsUser && !bIsUser) {
      next[b] = emptyField({ locked: true });
    }

    if (bIsUser && !aIsUser) {
      next[a] = emptyField({ locked: true });
    }
  }

  return next;
}
