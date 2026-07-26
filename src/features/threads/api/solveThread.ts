import type { ThreadKey } from "../domain/threadForm";
import type { ThreadCalculationInput } from "./types";
import { solveThreadApi } from "./client";

export async function solveThread(
  input: ThreadCalculationInput,
): Promise<Partial<Record<ThreadKey, number>>> {
  const result = await solveThreadApi(input);

  return {
    drill_diameter: result.drillDiameterMm,
    thread_depth: result.threadDepthMm,
  };
}
