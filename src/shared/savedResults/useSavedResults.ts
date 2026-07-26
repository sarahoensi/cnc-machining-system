import { useFeatureForm } from "@app/providers/FormStateProvider";

import type { SavedResultEntry, SavedResultForm } from "./types";

type Options<TForm extends SavedResultForm> = {
  storageKey: string;
  normalizeLoadedForm?: (form: TForm) => TForm;
};

export function useSavedResults<TForm extends SavedResultForm>({
  storageKey,
  normalizeLoadedForm,
}: Options<TForm>) {
  const [history, setHistory] = useFeatureForm<SavedResultEntry<TForm>[]>(
    storageKey,
    () => [],
  );

  function save(form: TForm) {
    if (form.status !== "solved") return;

    setHistory((prev) => [...prev, createSavedResultEntry(form)]);
  }

  function load(entry: SavedResultEntry<TForm>) {
    const form = cloneSavedResultForm(entry.form);
    return normalizeLoadedForm ? normalizeLoadedForm(form) : form;
  }

  function remove(id: string) {
    setHistory((prev) => prev.filter((entry) => entry.id !== id));
  }

  function clear() {
    setHistory([]);
  }

  return {
    history,
    save,
    load,
    remove,
    clear,
  };
}

export function createSavedResultEntry<TForm extends SavedResultForm>(
  form: TForm,
  options?: {
    id?: string;
    createdAt?: number;
  },
): SavedResultEntry<TForm> {
  return {
    id: options?.id ?? crypto.randomUUID(),
    form: cloneSavedResultForm(form),
    createdAt: options?.createdAt ?? Date.now(),
  };
}

export function cloneSavedResultForm<TForm extends SavedResultForm>(
  form: TForm,
): TForm {
  return structuredClone(form);
}
