// features/finishing/api/client.ts

import { tauriInvoke } from "@shared/api/tauriClient";
import {
  GenerateFinishingPlanRequest,
  RegisterFinishingMeasurementRequest,
  FinishingExecutionResponse,
} from "./types";

/* ============================================================
   Generate plan
============================================================ */

export function generateFinishingPlanApi(request: GenerateFinishingPlanRequest) {
  return tauriInvoke<FinishingExecutionResponse>("generate_finishing_plan", {
    request,
  });
}

/* ============================================================
   Register measurement
============================================================ */

export function registerFinishingMeasurementApi(
  request: RegisterFinishingMeasurementRequest,
) {
  return tauriInvoke<FinishingExecutionResponse>("register_finishing_measurement", {
    request,
  });
}
