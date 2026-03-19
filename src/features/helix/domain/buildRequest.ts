// features/helix/domain/builRequest.ts

import type { HelixKey } from "./helixForm";
import type { SolveHelixRequest } from "../api/types";

export function buildHelixRequest(
  input: Partial<Record<HelixKey, number>>,
  mode: "Inner" | "Outer"
): SolveHelixRequest {

  const {
    diameter,
    tool_diameter,
    pitch,
    angle,
  } = input;

  if (pitch !== undefined) {
    return {
      type: "Pitch",
      mode,
      diameter: diameter!,
      tool_diameter: tool_diameter!,
      pitch,
    };
  }

  if (angle !== undefined) {
    return {
      type: "Angle",
      mode,
      diameter: diameter!,
      tool_diameter: tool_diameter!,
      angle,
    };
  }

  throw new Error("Invalid helix request state");
}