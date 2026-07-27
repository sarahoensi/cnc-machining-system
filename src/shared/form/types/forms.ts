// shared/form/types/forms.ts

import type { FieldState } from "@shared/form/types/fields";

export type FormStatus = "editing" | "solved";

export type FormState<K extends string, E = undefined> = {
  status: FormStatus;
  fields: Record<K, FieldState>;
  extras: E;

  formError?: string | string[];
};
