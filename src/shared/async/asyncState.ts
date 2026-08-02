// shared/types/async.ts

/**
 * Generic lifecycle state for async operations.
 */

export type AsyncStatus = "idle" | "running" | "success" | "error";

/**
 * Generic async state container.
 */

export type AsyncState<T> =
  | { status: "idle" }
  | { status: "running" }
  | { status: "success"; data: T }
  | { status: "error"; error: string };

/* ============================================================
   Helpers
============================================================ */

export function idle<T>(): AsyncState<T> {
  return { status: "idle" };
}

export function running<T>(): AsyncState<T> {
  return { status: "running" };
}

export function success<T>(data: T): AsyncState<T> {
  return { status: "success", data };
}

export function failure<T>(error: string): AsyncState<T> {
  return { status: "error", error };
}

export type Nullable<T> = T | null;
