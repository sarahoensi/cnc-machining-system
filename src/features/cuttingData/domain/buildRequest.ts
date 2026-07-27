// features/cutting_data/domain/buildRequest.ts

import { SolveCuttingDataRequest } from "../api/types";
import { CuttingDataKey } from "./cuttingDataForm";

export function buildCuttingDataRequest(
  input: Partial<Record<CuttingDataKey, number>>,
): SolveCuttingDataRequest {
  return {
    diameter_mm: input.diameter,
    teeth: input.teeth,
    cutting_speed_m_per_min: input.cutting_speed,
    rpm: input.rpm,
    feed_rate_mm_per_min: input.feed_rate,
    chip_load_mm_per_tooth: input.chip_load,
  };
}
