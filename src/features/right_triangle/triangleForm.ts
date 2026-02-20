// features/right_triangle/triangleForm.ts

import type { FormState } from "@shared/types/forms";
import { emptyField } from "@shared/types/fields";

export type TriangleKey =
  | "a"
  | "b"
  | "c"
  | "alpha"
  | "beta"
  | "gamma";

export type TriangleExtras = {
  // hvis du trenger noe senere (f.eks type triangle)
};

export function createInitialTriangleForm(): FormState<
  TriangleKey,
  TriangleExtras
> {
  return {
    status: "editing",
    fields: {
      a: emptyField(),
      b: emptyField(),
      c: emptyField(),
      alpha: emptyField(),
      beta: emptyField(),
      gamma: emptyField(),
    },
    extras: {},
  };
}