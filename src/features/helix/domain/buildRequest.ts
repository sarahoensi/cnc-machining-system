// features/helix/domain/builRequest.ts

import type { HelixKey } from "./helixForm";
import type { SolveHelixRequest } from "../api/types";

export function buildHelixRequest(
  input: Partial<Record<HelixKey, number>>,
  mode: "Inner" | "Outer"
): SolveHelixRequest{

  const {
    diameter: diameter,
    tool_diameter: tool_diameter,
    pitch: pitch,
    angle: angle,
  } = input;

  if (!diameter || !tool_diameter)
    throw new Error("Missing required values");

  if (pitch !== undefined) {
    return {
      type: "Pitch",
      mode,
      diameter: diameter,
      tool_diameter: tool_diameter,
      pitch: pitch,
    };
  }

  if (angle !== undefined) {
    return {
      type: "Angle",
      mode,
      diameter: diameter,
      tool_diameter: tool_diameter,
      angle: angle,
    };
  }

  throw new Error("Either pitch or angle must be provided");
}