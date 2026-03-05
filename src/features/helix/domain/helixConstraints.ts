// features/helix/domain/helixConstraints.ts

import type { HelixKey } from "./helixForm";

export const validHelixInputSets: readonly (readonly HelixKey[])[] = [
  ["diameter", "toolDiameter", "pitch"],
  ["diameter", "toolDiameter", "angle"],
] as const;

export const mutuallyExclusiveHelixPairs: readonly (readonly [HelixKey, HelixKey])[] = [
  ["pitch", "angle"],
] as const;