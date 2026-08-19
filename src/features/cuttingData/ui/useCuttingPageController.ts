// features/cuttingData/ui/useCuttingPageController.ts

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { emptyField } from "@shared/form/types/fields";
import { handleCalculateAsync, handleUserEdit } from "@shared/form/engine/formEngine";
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
import {
  getRequiredInputKeys,
  type CuttingApprenticeTarget,
  useCuttingApprenticeController,
} from "./apprentice/useCuttingApprenticeController";
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
  const [form, setForm] = useFeatureForm("cutting", createInitialCuttingDataForm);

  const savedResults = useSavedResults<ReturnType<typeof createInitialCuttingDataForm>>(
    {
      storageKey: "cutting-history",
    },
  );
  const apprentice = useCuttingApprenticeController({
    form,
    onTargetChange: resetFormForApprenticeTarget,
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
    const hasInlineError = focusOrder.some((key) => Boolean(next.fields[key].error));

    if (hasInlineError) {
      navigation.focusFirstInvalidAfterRender((key) => Boolean(next.fields[key].error));
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

  function resetFormForApprenticeTarget(target: CuttingApprenticeTarget) {
    setForm((prev) => {
      const requiredKeys = new Set(getRequiredInputKeys(target));
      const nextFields = { ...prev.fields };

      for (const key of Object.keys(nextFields) as CuttingDataKey[]) {
        const field = nextFields[key];
        const shouldKeepUserInput = requiredKeys.has(key) && field.source === "user";

        nextFields[key] = shouldKeepUserInput ? field : emptyField();
      }

      return {
        status: "editing",
        fields: nextFields,
        extras: prev.extras,
        formError: undefined,
      };
    });
  }

  return {
    form,
    navigation,
    onFieldChange,
    calculate,
    resetForm,
    apprentice,

    history: savedResults.history,

    save,
    load,
    remove: savedResults.remove,
    clear: savedResults.clear,
  };
}
