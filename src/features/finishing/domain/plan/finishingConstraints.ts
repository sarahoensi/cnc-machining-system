// features/finishing/domain/plan/finishingConstraints.ts

import type { FinishingKey } from "./finishingForm";

export const validFinishingInputSets: readonly (readonly FinishingKey[])[] = [
  ["start_diameter", "target_diameter", "cuts"],
  ["start_diameter", "target_diameter", "radial_engagement"],
] as const;

export const mutuallyExclusiveFinishingPairs: readonly (
  readonly [FinishingKey, FinishingKey]
)[] = [
  ["cuts", "radial_engagement"],
] as const;