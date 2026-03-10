// features/finishing/domain/execution/buildRegisterRequest.ts

import type {
  RegisterFinishingMeasurementRequest,
} from "../../api/types";

export function buildRegisterRequest(
  executionId: string,
  step: number,
  measurement: number
): RegisterFinishingMeasurementRequest {

  if (!executionId)
    throw new Error("Missing execution id");

  if (step <= 0)
    throw new Error("Invalid step number");

  if (!Number.isFinite(measurement))
    throw new Error("Invalid measurement");

  return {
    execution_id: executionId,
    step_number: step,
    measurement_mm: measurement,
  };
}