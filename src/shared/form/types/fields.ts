// shared/form/types/fields.ts

export type FieldKind = "input" | "result";

export type FieldSource = "empty" | "user" | "machine";

export type FieldState = {
  value: string;
  source: FieldSource;
  kind: FieldKind;
  locked: boolean;
  invalid: boolean;

  machineValue?: number;
  error?: string;
};

/* ---------------------------------- */
/* Base factories                     */
/* ---------------------------------- */

export const emptyField = (overrides?: Partial<FieldState>): FieldState => ({
  value: "",
  source: "empty",
  kind: "input",
  locked: false,
  invalid: false,
  machineValue: undefined,
  error: undefined,
  ...overrides,
});

export const userField = (
  value: string,
  overrides?: Partial<FieldState>,
): FieldState => ({
  value,
  source: value === "" ? "empty" : "user",
  kind: "input",
  locked: false,
  invalid: false,
  error: undefined,
  ...overrides,
});

export const machineField = (
  value: string,
  overrides?: Partial<FieldState>,
): FieldState => ({
  value,
  source: "machine",
  kind: "input",
  locked: false,
  invalid: false,
  ...overrides,
});

export const resultField = (overrides?: Partial<FieldState>): FieldState => ({
  value: "",
  source: "empty",
  kind: "result",
  locked: true,
  invalid: false,
  machineValue: undefined,
  error: undefined,
  ...overrides,
});

export const machineResultField = (
  value: string,
  overrides?: Partial<FieldState>,
): FieldState => ({
  value,
  source: "machine",
  kind: "result",
  locked: false,
  invalid: false,
  ...overrides,
});
