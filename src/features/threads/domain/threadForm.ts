import type { FormState } from "@shared/form/types/forms";
import { resultField, userField } from "@shared/form/types/fields";

import type { ThreadOptionsResponse, ThreadType } from "../api/types";
import { emptyThreadOptions } from "./threadOptions";

export type ThreadKey =
  | "size"
  | "pitch"
  | "drill_diameter"
  | "thread_depth";

export type ThreadExtras = {
  type: ThreadType;
  options: ThreadOptionsResponse;
  loadingOptions: boolean;
};

export type ThreadFormState = FormState<ThreadKey, ThreadExtras>;

export function createInitialThreadForm(): ThreadFormState {
  return {
    status: "editing",
    fields: {
      size: userField(""),
      pitch: userField(""),
      drill_diameter: resultField(),
      thread_depth: resultField(),
    },
    extras: {
      type: "metric",
      options: emptyThreadOptions,
      loadingOptions: true,
    },
  };
}
