import type { HelixKey } from "./helixForm";
import type { SolveHelixRequest } from "../api/types";

export function buildHelixRequest(
  input: Partial<Record<HelixKey, number>>,
  mode: "Inner" | "Outer"
): SolveHelixRequest{

  const {
    diameter,
    toolDiameter,
    pitch,
    angle,
  } = input;

  if (!diameter || !toolDiameter)
    throw new Error("Missing required values");

  if (pitch !== undefined) {
    return {
      type: "Pitch",
      mode,
      diameter: diameter,
      tool_diameter: toolDiameter,
      pitch: pitch,
    };
  }

  if (angle !== undefined) {
    return {
      type: "Angle",
      mode,
      diameter: diameter,
      tool_diameter: toolDiameter,
      angle: angle,
    };
  }

  throw new Error("Either pitch or angle must be provided");
}