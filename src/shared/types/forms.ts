// shared/types/forms.ts

import type { FieldState } from "@shared/types/fields";

export type FormMode =
  | "editing"
  | "solved";


  export type FormState<K extends string> = {
  mode: FormMode;
  fields: Record<K, FieldState>;
};