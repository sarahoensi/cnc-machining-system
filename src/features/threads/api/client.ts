import { tauriInvoke } from "@shared/api/tauriClient";

import type {
  ThreadCalculationInput,
  ThreadCalculationResult,
  ThreadOptionsResponse,
} from "./types";

export function listThreadOptionsApi() {
  return tauriInvoke<ThreadOptionsResponse>("list_thread_options");
}

export function solveThreadApi(request: ThreadCalculationInput) {
  return tauriInvoke<ThreadCalculationResult>("solve_thread", { request });
}
