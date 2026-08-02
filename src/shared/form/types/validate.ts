// shared/form/types/validate.ts

import type { FieldState } from "@shared/form/types/fields";

export type FormValidateFn<K extends string, E> = (
  fields: Record<K, FieldState>,
  extras: E,
) => string[] | null;
