// features/cutting_data/api/solveCuttingData.ts

import { buildCuttingDataRequest } from "../domain/buildRequest";
import { solveCuttingDataApi } from "./client";
import type { CuttingDataKey } from "../domain/cuttingDataForm";

export async function solveCuttingData(
  input: Partial<Record<CuttingDataKey, number>>,
): Promise<Partial<Record<CuttingDataKey, number>>> {

  const request = buildCuttingDataRequest(input);
  const result = await solveCuttingDataApi(request);

  const map: Record<string, CuttingDataKey> = {
    cutting_speed_m_per_min: "cutting_speed",
    rpm: "rpm",
    feed_rate_mm_per_min: "feed_rate",
    chip_load_mm_per_tooth: "chip_load",
  };

  const output: Partial<Record<CuttingDataKey, number>> = {};

  for (const key in map) {
    const value = (result as any)[key];
    if (value !== undefined) {
      output[map[key]] = value;
    }
  }

  return output;
}