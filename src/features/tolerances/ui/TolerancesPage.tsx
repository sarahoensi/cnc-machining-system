// features/tolerances/ui/toleranceMenu/useTolerancePageController.ts


import { useFeatureForm } from "@app/providers/FormStateProvider";
import { useSavedResults } from "@shared/savedResults";
import {
  handleCalculateAsync,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { solveTolerance } from "../api/solveTolerance";
import type { ToleranceMode, ToleranceObjectType } from "../api/types";

import {
  createInitialToleranceForm,
  type ToleranceFormState,
  type ToleranceKey,
} from "../domain/toleranceForm";
import { parseTolerance } from "../domain/parseTolerance";
import { validateToleranceForm } from "../domain/validateToleranceForm";
import {
  preserveEquivalentToleranceSelection,
  reconcileSelectionFields,
} from "../domain/toleranceOptions";
import {
  applyToleranceGradeChange,
  applyToleranceLetterChange,
  applyToleranceUserEdit,
} from "../domain/toleranceSelection";

import { useToleranceOptions } from "./toleranceMenu/useToleranceOptions";
import { useValidToleranceGrades } from "./toleranceMenu/useValidToleranceGrades";
import { useToleranceOptionsLoader } from "./toleranceMenu/useToleranceOptionsLoader";


export function useTolerancePageController() {
  const [form, setForm] = useFeatureForm(
    "tolerances",
    createInitialToleranceForm,
  );

  const savedResults = useSavedResults<ToleranceFormState>({
    storageKey: "tolerances-history",
  });

  const { mode, options, loadingOptions } = form.extras;

  const toleranceOptions = useToleranceOptions(form);

  useToleranceOptionsLoader(setForm);

  useValidToleranceGrades(
    setForm,
    toleranceOptions.holeGrade,
    toleranceOptions.holeGrades,
    toleranceOptions.shaftGrade,
    toleranceOptions.shaftGrades,
  );

  function onModeChange(value: ToleranceMode) {
    setForm((prev) => {
      const next = handleModeChange(prev, {
        ...prev.extras,
        mode: value,
      });

      return {
        ...next,
        fields: preserveEquivalentToleranceSelection(next, prev.extras.mode),
      };
    });
  }

  function onFieldChange(key: ToleranceKey, value: string) {
    setForm((prev) => applyToleranceUserEdit(prev, key, value));
  }

  function onToleranceLetterChange(
    feature: ToleranceObjectType,
    value: string,
  ) {
    setForm((prev) =>
      applyToleranceLetterChange(prev, options, feature, value),
    );
  }

  function onToleranceGradeChange(
    feature: ToleranceObjectType,
    value: string,
  ) {
    setForm((prev) =>
      applyToleranceGradeChange(prev, feature, value),
    );
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseTolerance,
      solveTolerance,
      validateToleranceForm,
    );

    setForm(next);
    return next;
  }

  function resetForm() {
    setForm((prev) => {
      const initial = createInitialToleranceForm();

      return {
        ...initial,
        fields: reconcileSelectionFields(initial.fields, prev.extras.options),
        extras: {
          ...initial.extras,
          options: prev.extras.options,
          loadingOptions: prev.extras.loadingOptions,
        },
      };
    });
  }

  function save() {
    savedResults.save(form);
  }

  function load(entry: (typeof savedResults.history)[number]) {
    setForm(savedResults.load(entry));
  }

  return {
    form,

    mode,
    loadingOptions,

    ...toleranceOptions,

    onModeChange,
    onFieldChange,
    onToleranceLetterChange,
    onToleranceGradeChange,
    calculate,
    resetForm,

    history: savedResults.history,
    save,
    load,
    remove: savedResults.remove,
    clear: savedResults.clear,
  };
}
