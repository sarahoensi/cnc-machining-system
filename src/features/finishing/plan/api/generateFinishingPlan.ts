// features/finishing/api/generateFinishingPlan.ts

import { generateFinishingPlanApi } from "../../api/finishingApi";

import type { FinishingExecutionResponse } from "../../api/types";
import { buildFinishingRequest } from "../domain/buildGenerateRequest";
import { FinishingExtras, FinishingKey } from "../domain/finishingForm";

export async function generateFinishingPlan(
  input: Partial<Record<FinishingKey, number>>,
  extras: FinishingExtras
): Promise<FinishingExecutionResponse> {

  const request = buildFinishingRequest(input, extras);

  const result = await generateFinishingPlanApi(request);

  return result;
}