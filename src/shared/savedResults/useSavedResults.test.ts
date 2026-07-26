import { describe, expect, it } from "vitest";

import {
  cloneSavedResultForm,
  createSavedResultEntry,
} from "./useSavedResults";

type TestForm = {
  status: "editing" | "solved";
  fields: {
    value: {
      value: string;
    };
  };
};

function createSolvedForm(): TestForm {
  return {
    status: "solved",
    fields: {
      value: {
        value: "42",
      },
    },
  };
}

describe("savedResults helpers", () => {
  it("creates deterministic entries when id and timestamp are provided", () => {
    const form = createSolvedForm();

    const entry = createSavedResultEntry(form, {
      id: "entry-1",
      createdAt: 123,
    });

    expect(entry).toEqual({
      id: "entry-1",
      createdAt: 123,
      form,
    });
  });

  it("clones forms so history entries are not mutated by later edits", () => {
    const form = createSolvedForm();
    const entry = createSavedResultEntry(form, {
      id: "entry-1",
      createdAt: 123,
    });

    form.fields.value.value = "99";

    expect(entry.form.fields.value.value).toBe("42");
  });

  it("clones loaded forms", () => {
    const form = createSolvedForm();
    const cloned = cloneSavedResultForm(form);

    cloned.fields.value.value = "99";

    expect(form.fields.value.value).toBe("42");
  });
});
