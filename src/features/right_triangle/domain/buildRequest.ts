// --------------------------------------------------
// Request builder (feature logic)
// --------------------------------------------------

import { SolveRightTriangleRequest } from "../api/types";
import { TriangleKey } from "./triangleForm";

export function buildRequest(
  input: Partial<Record<TriangleKey, number>>
): SolveRightTriangleRequest {

  const { a, b, c, alpha, beta } = input;

  // SIDE + SIDE
  if (a !== undefined && b !== undefined)
    return { type: "Legs", a_mm: a, b_mm: b };

  if (a !== undefined && c !== undefined)
    return { type: "LegAAndHypotenuse", a_mm: a, c_mm: c };

  if (b !== undefined && c !== undefined)
    return { type: "LegBAndHypotenuse", b_mm: b, c_mm: c };

  // SIDE + ANGLE
  if (a !== undefined && alpha !== undefined)
    return { type: "LegAAndAlpha", a_mm: a, alpha_deg: alpha };

  if (a !== undefined && beta !== undefined)
    return { type: "LegAAndBeta", a_mm: a, beta_deg: beta };

  if (b !== undefined && alpha !== undefined)
    return { type: "LegBAndAlpha", b_mm: b, alpha_deg: alpha };

  if (b !== undefined && beta !== undefined)
    return { type: "LegBAndBeta", b_mm: b, beta_deg: beta };

  if (c !== undefined && alpha !== undefined)
    return { type: "HypotenuseAndAlpha", c_mm: c, alpha_deg: alpha };

  if (c !== undefined && beta !== undefined)
    return { type: "HypotenuseAndBeta", c_mm: c, beta_deg: beta };

  throw new Error("Ugyldig kombinasjon av input");
}