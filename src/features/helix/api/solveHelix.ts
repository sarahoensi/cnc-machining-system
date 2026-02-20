import { buildHelixRequest } from "../domain/buildRequest";
import { solveHelixApi } from "./client";
import type { HelixKey } from "../domain/helixForm";

export async function solveHelix(
  input: Partial<Record<HelixKey, number>>,
  mode: "Inner" | "Outer"
) {
  const request = buildHelixRequest(input, mode);

  const result = await solveHelixApi(request);

  return {
    
    pitch: result.pitch_mm_per_rev,
    angle: result.angle_deg,
    
  };
}