// shared/api/tauriClient.ts

/**
 * Thin wrapper around Tauri's invoke API.
 *
 * Centralizes error handling and serialization concerns.
 */


import { invoke } from "@tauri-apps/api/core";

export async function tauriInvoke<T>(command: string, payload?: Record<string, unknown>) {
  return invoke<T>(command, payload);
}
