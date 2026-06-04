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

    const entry: SavedResultEntry<TForm> = {
      id: crypto.randomUUID(),
      form: structuredClone(form),
      createdAt: Date.now(),
    };

    setHistory((prev) => [...prev, entry]);
  }

  function load(entry: SavedResultEntry<TForm>) {
    const form = structuredClone(entry.form);
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
