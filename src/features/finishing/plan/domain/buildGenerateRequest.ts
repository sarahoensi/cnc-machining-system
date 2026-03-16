// features/finishing/domain/plan/buildFinishingRequest.ts

import type { GenerateFinishingPlanRequest } from "../../api/types";
import { FinishingExtras, FinishingKey } from "./finishingForm";

export function buildFinishingRequest(
  input: Partial<Record<FinishingKey, number>>,
  extras: FinishingExtras
): GenerateFinishingPlanRequest {

  const {
    start_diameter_mm,
    target_diameter_mm,
    cuts,
    radial_engagement_mm,
  } = input;

  if (start_diameter_mm == null || target_diameter_mm == null) {
    throw new Error("Missing required diameters");
  }

  if (cuts !== undefined) {

    return {
      type: "ByCuts",
      mode: extras.mode,
      start_diameter_mm: start_diameter_mm,
      target_diameter_mm: target_diameter_mm,
      cuts,
    };

  }

  if (radial_engagement_mm !== undefined) {

    return {
      type: "ByRadialEngagement",
      mode: extras.mode,
      start_diameter_mm: start_diameter_mm,
      target_diameter_mm: target_diameter_mm,
      radial_engagement_mm: radial_engagement_mm,
    };

  }

  throw new Error("Either cuts or radial engagement must be provided");
}