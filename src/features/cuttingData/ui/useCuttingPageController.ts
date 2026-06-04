// features/cuttingData/ui/useCuttingPageController.ts

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { useSavedResults } from "@shared/savedResults";

import { createInitialCuttingDataForm } from "../domain/cuttingDataForm";

export function useCuttingPageController() {
  const [form, setForm] = useFeatureForm(
    "cutting",
    createInitialCuttingDataForm,
  );

  const savedResults = useSavedResults<
    ReturnType<typeof createInitialCuttingDataForm>
  >({
    storageKey: "cutting-history",
  });

  function save() {
    savedResults.save(form);
  }

  function load(entry: (typeof savedResults.history)[number]) {
    setForm(savedResults.load(entry));
  }


  function resetForm() {
    setForm(createInitialCuttingDataForm());
  }

  return {
    form,
    setForm,

    history: savedResults.history,

    save,
    load,
    remove: savedResults.remove,
    clear: savedResults.clear,
    resetForm,
  };
}
