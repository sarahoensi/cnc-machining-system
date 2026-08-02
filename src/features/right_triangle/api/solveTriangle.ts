// features/right_triangle/solveTriangle.ts

import { TriangleKey } from "../domain/triangleForm";
import { solveRightTriangle } from "./client";
import { buildRequest } from "../domain/buildRequest";

export async function solveTriangle(
  input: Partial<Record<TriangleKey, number>>,
): Promise<Partial<Record<TriangleKey, number>>> {
  const request = buildRequest(input);

  const result = await solveRightTriangle(request);

  return {
    a: result.a_mm,
    b: result.b_mm,
    c: result.c_mm,
    alpha: result.alpha_deg,
    beta: result.beta_deg,
  };
}
