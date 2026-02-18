// shared/types/forms.ts

import type { FieldState } from "@shared/types/fields";

export type FormStatus =
  | "editing"
  | "solved"
  | "executing";


  export type FormState<K extends string, E = undefined> = {
  status: FormStatus;
  fields: Record<K, FieldState>;
  extras: E;
};