// features/finishing/domain/execution/buildRegisterRequest.ts

import type {
  RegisterFinishingMeasurementRequest,
} from "../../api/types";

export function buildRegisterRequest(
  step: number,
  measurement: number
): RegisterFinishingMeasurementRequest {


  if (step <= 0)
    throw new Error("Invalid step number");

  if (!Number.isFinite(measurement))
    throw new Error("Invalid measurement");

  return {
    step_number: step,
    measurement_mm: measurement,
  };
}