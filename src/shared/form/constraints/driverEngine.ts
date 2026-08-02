// shared/form/constraints/driverEngine.ts

import type { FieldState } from "@shared/form/types/fields";
import { evaluateConstraints } from "./constraintLogic";
import { applyPairLogic } from "./pairLogic";

/**
 * Unified driver engine.
 *
 * Combines:
 * - Structural constraint sets
 * - Independent driver pairs
 *
 * Rules:
 * - Only active in "editing" mode
 * - Locked fields are always empty
 * - editedKey wins on conflict
 */
export function applyDriverEngine<K extends string>(
  fields: Record<K, FieldState>,
  options: {
    validSets: readonly (readonly K[])[];
    pairs?: readonly (readonly [K, K])[];
    editedKey: K | null;
    mode: "editing" | "solved";
  },
): {
  fields: Record<K, FieldState>;
  activeSet: readonly K[] | null;
} {
  const { validSets, pairs = [], editedKey, mode } = options;

  // In solved mode, driver logic does not change fields.
  if (mode === "solved") {
    return {
      fields,
      activeSet: null,
    };
  }

  // 1. Structural constraints.
  const structural = evaluateConstraints(fields, validSets, editedKey);

  // 2. Pair constraints.
  const paired = applyPairLogic(structural.fields, pairs, editedKey, "editing");

  return {
    fields: paired,
    activeSet: structural.activeSet,
  };
}
