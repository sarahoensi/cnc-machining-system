// features/right_triangle/triangleForm.ts

import type { FormState } from "@shared/form/types/forms";
import { emptyField } from "@shared/form/types/fields";

export type TriangleKey = "a" | "b" | "c" | "alpha" | "beta";

export type TriangleExtras = {
  // hvis du trenger noe senere (f.eks type triangle)
};

export function createInitialTriangleForm(): FormState<TriangleKey, TriangleExtras> {
  return {
    status: "editing",
    fields: {
      a: emptyField(),
      b: emptyField(),
      c: emptyField(),
      alpha: emptyField(),
      beta: emptyField(),
    },
    extras: {},
  };
}
