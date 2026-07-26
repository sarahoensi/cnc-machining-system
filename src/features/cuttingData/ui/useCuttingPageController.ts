// features/cuttingData/ui/useCuttingPageController.ts

import { useFeatureForm } from "@app/providers/FormStateProvider";
import {
  handleCalculateAsync,
  handleUserEdit,
} from "@shared/form/engine/formEngine";
import { useFormNavigation } from "@shared/hooks";
import { useSavedResults } from "@shared/savedResults";

import { solveCuttingData } from "../api/solveCuttingData";
import {
  mutuallyExclusiveCuttingDataPairs,
  validCuttingDataInputSets,
} from "../domain/cuttingDataConstraints";
import {
  createInitialCuttingDataForm,
  type CuttingDataKey,
} from "../domain/cuttingDataForm";
import { parseCuttingData } from "../domain/parseCuttingData";
import { validateCuttingDataForm } from "../domain/validateCuttingForm";
import { cuttingDataFieldConfig } from "./cuttingDataFieldConfig";

const focusOrder: CuttingDataKey[] = [
  "diameter",
  "rpm",
  "cutting_speed",
  "teeth",
  "feed_rate",
  "chip_load",
];

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

  const navigation = useFormNavigation({
    keys: cuttingDataFieldConfig.map((fieldConfig) => fieldConfig.key),
    autoFocusOnMount: true,
    activePath: "/cutting",
    onSubmit: calculate,
  });

  function onFieldChange(key: CuttingDataKey, value: string) {
    setForm((prev) =>
      handleUserEdit(
        prev,
        key,
        value,
        validCuttingDataInputSets,
        mutuallyExclusiveCuttingDataPairs,
      ),
    );
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseCuttingData,
      (input) => solveCuttingData(input),
      validateCuttingDataForm,
    );

    setForm(next);
    const hasInlineError = focusOrder.some((key) =>
      Boolean(next.fields[key].error),
    );

    if (hasInlineError) {
      navigation.focusFirstInvalidAfterRender((key) =>
        Boolean(next.fields[key].error),
      );
      return;
    }

    if (!next.formError) return;

    navigation.focusFirstInOrderAfterRender(focusOrder, (key) => {
      const value = next.fields[key]?.value;
      return value == null || String(value).trim() === "";
    });
  }

  function save() {
    savedResults.save(form);
  }

  function load(entry: (typeof savedResults.history)[number]) {
    setForm(savedResults.load(entry));
  }


  function resetForm() {
    setForm(createInitialCuttingDataForm());
    navigation.focusFirstAfterRender();
  }

  return {
    form,
    navigation,
    onFieldChange,
    calculate,
    resetForm,

    history: savedResults.history,

    save,
    load,
    remove: savedResults.remove,
    clear: savedResults.clear,
  };
}
