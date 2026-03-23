// features/cuttingData/page/useCuttingPageController.ts

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { createInitialCuttingDataForm } from "../domain/cuttingDataForm";

type SavedEntry = {
  id: string;
  form: ReturnType<typeof createInitialCuttingDataForm>;
  createdAt: number;
};

export function useCuttingPageController() {

  const [form, setForm] = useFeatureForm(
    "cutting",
    createInitialCuttingDataForm
  );

  const [history, setHistory] = useFeatureForm<SavedEntry[]>(
    "cutting-history",
    () => []
  );

  /* =========================
     Save
  ========================= */

  function save() {
    if (form.status !== "solved") return;

    const entry: SavedEntry = {
      id: crypto.randomUUID(),
      form: structuredClone(form),
      createdAt: Date.now(),
    };

    setHistory(prev => [...prev, entry]);
  }

  /* =========================
     Load
  ========================= */

  function load(entry: SavedEntry) {
    setForm(structuredClone(entry.form));
  }

  /* =========================
     Delete
  ========================= */

  function remove(id: string) {
    setHistory(prev => prev.filter(e => e.id !== id));
  }

  /* =========================
     Clear
  ========================= */

  function clear() {
    setHistory([]);
  }

  /* =========================
     Reset
  ========================= */

  function resetForm() {
    setForm(createInitialCuttingDataForm());
  }

  return {
    form,
    setForm,

    history,

    save,
    load,
    remove,
    clear,
    resetForm,
  };
}