import { useFeatureForm } from "@app/providers/FormStateProvider";
import {
  handleCalculateAsync,
  handleUserEdit,
} from "@shared/form/engine/formEngine";
import { useCalculatorFormNavigation } from "@shared/hooks";

import { solveTriangle } from "../api/solveTriangle";
import {
  mutuallyExclusiveTrianglePairs,
  validTriangleInputSets,
} from "../domain/triangleConstraints";
import {
  createInitialTriangleForm,
  type TriangleKey,
} from "../domain/triangleForm";
import { parseTriangle } from "../domain/parseTriangle";
import { validateTriangleForm } from "../domain/validateTriangleForm";
import { triangleFieldConfig } from "./triangleFieldConfig";

export function useTrianglePageController() {
  const [form, setForm] = useFeatureForm(
    "triangle",
    createInitialTriangleForm,
  );

  const fieldOrder = triangleFieldConfig.map((fieldConfig) => fieldConfig.key);
  const navigation = useCalculatorFormNavigation({
    fieldOrder,
    activePath: "/triangle",
    onSubmit: calculate,
    trackActiveField: true,
  });

  function onFieldChange(key: TriangleKey, value: string) {
    setForm((prev) =>
      handleUserEdit(
        prev,
        key,
        value,
        validTriangleInputSets,
        mutuallyExclusiveTrianglePairs,
      ),
    );
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseTriangle,
      solveTriangle,
      validateTriangleForm,
    );

    setForm(next);
    navigation.focusAfterCalculate(next);
  }

  function resetForm() {
    setForm(createInitialTriangleForm());
    navigation.focusAfterReset();
  }

  return {
    form,
    activeField: navigation.activeField,
    navigation,
    onFieldChange,
    calculate,
    resetForm,
  };
}
