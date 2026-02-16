// shared/core/drivers/pairLogic.ts

import type { FieldState } from "@shared/types/fields";

/**
 * Resolves locked keys for independent driver pairs
 * (e.g. Vc <-> n or F <-> fz).
 */
export function resolvePairLocks<
  F extends Record<string, FieldState>,
  K extends keyof F & string
>(
  fields: F,
  pairs: readonly (readonly [K, K])[]
): K[] {

  const locked: K[] = [];

  for (const [a, b] of pairs) {

    const aIsUser = fields[a].source === "user";
    const bIsUser = fields[b].source === "user";

    if (aIsUser && fields[b].source !== "machine") {
      locked.push(b);
    }

    if (bIsUser && fields[a].source !== "machine") {
      locked.push(a);
    }
  }

  return locked;
}
