// features/helix/api/clients.ts

import { tauriInvoke } from "@shared/api/tauriClient";
import {
  SolveHelixRequest,
  SolveHelixResponse,
} from "./types";

export function solveHelixApi(
  request: SolveHelixRequest
) {
  return tauriInvoke<SolveHelixResponse>(
    "solve_helix",
    { request }
  );
}