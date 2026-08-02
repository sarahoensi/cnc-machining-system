// features/cutting_data/api/client.ts

import { tauriInvoke } from "@shared/api/tauriClient";
import { SolveCuttingDataRequest, SolveCuttingDataResponse } from "./types";

export function solveCuttingDataApi(request: SolveCuttingDataRequest) {
  return tauriInvoke<SolveCuttingDataResponse>("solve_cutting_data", { request });
}
