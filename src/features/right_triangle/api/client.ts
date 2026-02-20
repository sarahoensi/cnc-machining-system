// features/right_triangle/api/client.ts

import { tauriInvoke } from "@shared/api/tauriClient";
import {
  SolveRightTriangleRequest,
  SolveRightTriangleResponse,
} from "./types";

export function solveRightTriangle(
  request: SolveRightTriangleRequest
) {
  return tauriInvoke<SolveRightTriangleResponse>(
    "solve_right_triangle",
    { request }
  );
}