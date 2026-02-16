// shared/types/fields.ts

export type FieldSource =
  | "empty"
  | "user"
  | "machine";

export type FieldState = {
  value: string;
  source: FieldSource;
};

export const emptyField = (): FieldState => ({
  value: "",
  source: "empty",
});

export const userField = (value: string): FieldState => ({
  value,
  source: value === "" ? "empty" : "user",
});

export const machineField = (value: string): FieldState => ({
  value,
  source: "machine",
});
