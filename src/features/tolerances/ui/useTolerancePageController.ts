// features/tolerances/ui/toleranceMenu/useTolerancePageController.ts

import { useEffect } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { useSavedResults } from "@shared/savedResults";
import { getTauriCommandError } from "@shared/api/tauriError";
import {
  handleCalculateAsync,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { listIso286ToleranceOptionsApi } from "../api/client";
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

import { getToleranceSelectState } from "./toleranceSelectState";

export function useTolerancePageController() {
  const [form, setForm] = useFeatureForm(
    "tolerances",
    createInitialToleranceForm,
  );

  const savedResults = useSavedResults<ToleranceFormState>({
    storageKey: "tolerances-history",
  });

  const { mode, loadingOptions } = form.extras;

  const toleranceSelects = getToleranceSelectState(form);

  useEffect(() => {
    let cancelled = false;

    async function loadOptions() {
      setForm((prev) => ({
        ...prev,
        extras: {
          ...prev.extras,
          loadingOptions: true,
        },
        formError: undefined,
      }));

      try {
        const response = await listIso286ToleranceOptionsApi();

        if (cancelled) return;

        setForm((prev) => ({
          ...prev,
          fields: reconcileSelectionFields(prev.fields, response),
          extras: {
            ...prev.extras,
            options: response,
            loadingOptions: false,
          },
        }));
      } catch (error) {
        if (cancelled) return;

        setForm((prev) => ({
          ...prev,
          extras: {
            ...prev.extras,
            loadingOptions: false,
          },
          formError: getToleranceErrorMessage(error),
        }));
      }
    }

    void loadOptions();

    return () => {
      cancelled = true;
    };
  }, [setForm]);

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
    setForm((prev) =>
      applyToleranceUserEdit(prev, key, value),
    );
  }

  function onToleranceLetterChange(
    feature: ToleranceObjectType,
    value: string,
  ) {
    setForm((prev) =>
      applyToleranceLetterChange(
        prev,
        prev.extras.options,
        feature,
        value,
      ),
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

    ...toleranceSelects,

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

function getToleranceErrorMessage(error: unknown) {
  const commandError = getTauriCommandError(error);

  if (commandError?.message) return commandError.message;
  if (typeof error === "string") return error;
  if (error instanceof Error && error.message) return error.message;

  return "ISO 286 calculation failed";
}