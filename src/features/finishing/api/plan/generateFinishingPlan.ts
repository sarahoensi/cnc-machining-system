// features/finishing/api/generateFinishingPlan.ts

import { generateFinishingPlanApi } from "../client";
import { buildFinishingRequest } from "../../domain/plan/buildGenerateRequest";

import type { FinishingKey, FinishingExtras } from "../../domain/plan/finishingForm";
import type { FinishingExecutionResponse } from "../types";

export async function generateFinishingPlan(
  input: Partial<Record<FinishingKey, number>>,
  extras: FinishingExtras
): Promise<FinishingExecutionResponse> {

  const request = buildFinishingRequest(input, extras);

  const result = await generateFinishingPlanApi(request);

  return result;
}