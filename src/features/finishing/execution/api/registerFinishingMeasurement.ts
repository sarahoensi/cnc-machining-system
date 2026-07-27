// features/finishing/api/registerFinishingMeasurement.ts

import { registerFinishingMeasurementApi } from "../../api/finishingApi";
import {
  RegisterFinishingMeasurementRequest,
  FinishingExecutionResponse,
} from "../../api/types";

export async function registerFinishingMeasurement(
  request: RegisterFinishingMeasurementRequest,
): Promise<FinishingExecutionResponse> {
  const result = await registerFinishingMeasurementApi(request);

  return result;
}
