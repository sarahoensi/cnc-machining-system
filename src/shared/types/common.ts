// shared/types/common.ts

/**
 * Represents the lifecycle state of a process or async action.
 *
 * Used for UI flows such as:
 * - Solve operations
 * - API calls
 * - Calculations
 *
 * This type is feature-agnostic and may be reused across modules.
 */
export type ProcessStatus =
  | "idle"      // Nothing has started yet
  | "running"   // Currently executing
  | "success"   // Finished successfully
  | "error";    // Finished with error


/**
 * Generic async state container.
 *
 * Useful when a feature needs to hold both
 * lifecycle state and typed result data.
 *
 * Example:
 *   AsyncState<TriangleResult>
 */
export type AsyncState<T> =
  | { status: "idle" }
  | { status: "running" }
  | { status: "success"; data: T }
  | { status: "error"; error: string };


/**
 * Generic identifier type.
 *
 * Can be used for entity IDs returned from backend.
 * Keeps flexibility in case backend changes ID type later.
 */
export type ID = string;


/**
 * Utility helper for optional values.
 *
 * Improves readability when expressing "nullable but intentional".
 */
export type Nullable<T> = T | null;
