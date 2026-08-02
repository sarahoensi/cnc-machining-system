// features/finishing/domain/plan/finishingConstraints.ts

import type { FinishingKey } from "./finishingForm";

export const validFinishingInputSets: readonly (readonly FinishingKey[])[] = [
  ["start_diameter_mm", "target_diameter_mm", "cuts"],
  ["start_diameter_mm", "target_diameter_mm", "radial_engagement_mm"],
] as const;

export const mutuallyExclusiveFinishingPairs: readonly (readonly [
  FinishingKey,
  FinishingKey,
])[] = [["cuts", "radial_engagement_mm"]] as const;
