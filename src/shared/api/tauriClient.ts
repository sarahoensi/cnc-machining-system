// shared/api/tauriClient.ts

/**
 * Thin wrapper around Tauri's invoke API.
 *
 * Centralizes error handling and serialization concerns.
 */


import { invoke } from "@tauri-apps/api/core";

export async function tauriInvoke<T>(
  command: string, 
  payload?: Record<string, unknown>) {

    console.log("TAURI CALL →", command, payload);

    const result = await invoke<T>(command, payload);

 console.log("TAURI RESULT ←", command, result);

  return result;

}
