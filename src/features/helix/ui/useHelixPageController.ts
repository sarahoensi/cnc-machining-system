import { useFeatureForm } from "@app/providers/FormStateProvider";
import {
  handleCalculateAsync,
  handleModeChange,
  handleUserEdit,
} from "@shared/form/engine/formEngine";
import { useCalculatorFormNavigation } from "@shared/hooks";

import { solveHelix } from "../api/solveHelix";
import {
  mutuallyExclusiveHelixPairs,
  validHelixInputSets,
} from "../domain/helixConstraints";
import {
  createInitialHelixForm,
  type HelixKey,
} from "../domain/helixForm";
import { parseHelix } from "../domain/parseHelix";
import { validateHelixForm } from "../domain/validateHelixForm";
import { helixFieldConfig } from "./helixFieldConfig";

export function useHelixPageController() {
  const [form, setForm] = useFeatureForm(
    "helix",
    createInitialHelixForm,
  );

  const fieldOrder = helixFieldConfig.map((fieldConfig) => fieldConfig.key);
  const navigation = useCalculatorFormNavigation({
    fieldOrder,
    activePath: "/helix",
    onSubmit: calculate,
    trackActiveField: true,
  });

  function onFieldChange(key: HelixKey, value: string) {
    setForm((prev) =>
      handleUserEdit(
        prev,
        key,
        value,
        validHelixInputSets,
        mutuallyExclusiveHelixPairs,
      ),
    );
  }

  function onModeChange(newMode: typeof form.extras.mode) {
    setForm((prev) =>
      handleModeChange(prev, {
        ...prev.extras,
        mode: newMode,
      }),
    );
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseHelix,
      (input) => solveHelix(input, form.extras.mode),
      validateHelixForm,
    );

    setForm(next);
    navigation.focusAfterCalculate(next);
  }

  function resetForm() {
    setForm(createInitialHelixForm());
    navigation.focusAfterReset();
  }

  return {
    form,
    activeField: navigation.activeField,
    navigation,
    onFieldChange,
    onModeChange,
    calculate,
    resetForm,
  };
}
