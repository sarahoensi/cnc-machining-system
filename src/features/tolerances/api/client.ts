import { tauriInvoke } from "@shared/api/tauriClient";
import type {
  Iso286MemberResult,
  LookupIso286ToleranceRequest,
  ToleranceOptionsResponse,
} from "./types";

export function lookupIso286ToleranceApi(request: LookupIso286ToleranceRequest) {
  return tauriInvoke<Iso286MemberResult>("lookup_iso286_tolerance", request);
}

export function listIso286ToleranceOptionsApi() {
  return tauriInvoke<ToleranceOptionsResponse>("list_iso286_tolerance_options");
}
