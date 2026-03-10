// features/finishing/domain/plan/buildFinishingRequest.ts

import type { FinishingKey, FinishingExtras } from "./finishingForm";
import type { GenerateFinishingPlanRequest } from "../../api/types";

export function buildFinishingRequest(
  input: Partial<Record<FinishingKey, number>>,
  extras: FinishingExtras
): GenerateFinishingPlanRequest {

  const {
    start_diameter,
    target_diameter,
    cuts,
    radial_engagement,
  } = input;

  if (start_diameter == null || target_diameter == null) {
    throw new Error("Missing required diameters");
  }

  if (cuts !== undefined) {

    return {
      type: "ByCuts",
      mode: extras.mode,
      start_diameter_mm: start_diameter,
      target_diameter_mm: target_diameter,
      cuts,
    };

  }

  if (radial_engagement !== undefined) {

    return {
      type: "ByRadialEngagement",
      mode: extras.mode,
      start_diameter_mm: start_diameter,
      target_diameter_mm: target_diameter,
      radial_engagement_mm: radial_engagement,
    };

  }

  throw new Error("Either cuts or radial engagement must be provided");
}