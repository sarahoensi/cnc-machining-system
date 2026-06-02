import { tauriInvoke } from "@shared/api/tauriClient";
import type {
  CalculateIso286FitRequest,
  Iso286FitResult,
} from "./types";

export function calculateIso286FitApi(request: CalculateIso286FitRequest) {
  return tauriInvoke<Iso286FitResult>("calculate_iso286_fit", request);
}
