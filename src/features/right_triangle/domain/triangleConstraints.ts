// features/right_triangle/domain/triangleConstraints.ts

import type { TriangleKey } from "./triangleForm";

/**
 * Gyldige input-kombinasjoner for en rettvinklet trekant.
 * Nøyaktig to verdier må oppgis.
 */
export const validTriangleInputSets: readonly (readonly [TriangleKey, TriangleKey])[] = [
  ["a", "b"],
  ["a", "alpha"],
  ["a", "beta"],
  ["b", "beta"],
  ["b", "alpha"],
  ["c", "alpha"],
  ["c", "beta"],
  ["c", "a"],
  ["c", "b"],
] as const;

/**
 * Felter som ikke kan brukes samtidig.
 */
export const mutuallyExclusiveTrianglePairs: readonly (readonly [TriangleKey, TriangleKey])[] = [
  ["alpha", "beta"],
] as const;