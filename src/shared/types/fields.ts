// shared/types/fields.ts

export type FieldSource =
  | "empty"
  | "user"
  | "machine";

export type FieldState = {
  value: string;
  source: FieldSource;
  locked: boolean;
  invalid: boolean;

  machineValue?: number;
};

/* ---------------------------------- */
/* Base factories                     */
/* ---------------------------------- */

export const emptyField = (
  overrides?: Partial<FieldState>
): FieldState => ({
  value: "",
  source: "empty",
  locked: false,
  invalid: false,
  ...overrides,
});

export const userField = (
  value: string,
  overrides?: Partial<FieldState>
): FieldState => ({
  value,
  source: value === "" ? "empty" : "user",
  locked: false,
  invalid: false,
  ...overrides,
});

export const machineField = (
  value: string,
  overrides?: Partial<FieldState>
): FieldState => ({
  value,
  source: "machine",
  locked: false,
  invalid: false,
  ...overrides,
});