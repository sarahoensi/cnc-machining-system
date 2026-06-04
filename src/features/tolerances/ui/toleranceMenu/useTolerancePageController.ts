// features/tolerances/ui/useTolerancePageController.ts

import { useEffect, useMemo } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { useSavedResults } from "@shared/savedResults";
import { getTauriCommandError } from "@shared/api/tauriError";
import {
  handleCalculateAsync,
  handleModeChange,
} from "@shared/form/engine/formEngine";

import { listIso286ToleranceOptionsApi } from "../../api/client";
import { solveTolerance } from "../../api/solveTolerance";
import type { ToleranceMode, ToleranceObjectType } from "../../api/types";

import {
  createInitialToleranceForm,
  type ToleranceFormState,
  type ToleranceKey,
} from "../../domain/toleranceForm";
import { parseTolerance } from "../../domain/parseTolerance";
import { validateToleranceForm } from "../../domain/validateToleranceForm";

import {
  gradesForZone,
  preserveEquivalentToleranceSelection,
  reconcileSelectionFields,
} from "../../domain/toleranceOptions";

import {
  applyToleranceGradeChange,
  applyToleranceLetterChange,
  applyToleranceUserEdit,
  patchSelectionFields,
} from "../../domain/toleranceSelection";

export function useTolerancePageController() {
  const [form, setForm] = useFeatureForm(
    "tolerances",
    createInitialToleranceForm,
  );

  const savedResults = useSavedResults<ToleranceFormState>({
    storageKey: "tolerances-history",
  });

  const { mode, options, loadingOptions } = form.extras;

  const holeLetter = form.fields.hole_letter.value;
  const holeGrade = form.fields.hole_grade.value;
  const shaftLetter = form.fields.shaft_letter.value;
  const shaftGrade = form.fields.shaft_grade.value;

  const holeGrades = useMemo(
    () => gradesForZone(options.holes, holeLetter),
    [holeLetter, options.holes],
  );

  const shaftGrades = useMemo(
    () => gradesForZone(options.shafts, shaftLetter),
    [shaftLetter, options.shafts],
  );

  const holeLetterOptions = useMemo(
    () =>
      options.holes.map((option) => ({
        value: option.zone,
        label: option.zone,
      })),
    [options.holes],
  );

  const shaftLetterOptions = useMemo(
    () =>
      options.shafts.map((option) => ({
        value: option.zone,
        label: option.zone,
      })),
    [options.shafts],
  );

  const holeGradeOptions = useMemo(
    () =>
      holeGrades.map((value) => ({
        value,
        label: value,
      })),
    [holeGrades],
  );

  const shaftGradeOptions = useMemo(
    () =>
      shaftGrades.map((value) => ({
        value,
        label: value,
      })),
    [shaftGrades],
  );

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

  useEffect(() => {
    if (holeGrades.length === 0 || holeGrades.includes(holeGrade)) return;

    setForm((prev) =>
      patchSelectionFields(prev, {
        hole_grade: holeGrades[0],
      }),
    );
  }, [holeGrade, holeGrades, setForm]);

  useEffect(() => {
    if (shaftGrades.length === 0 || shaftGrades.includes(shaftGrade)) return;

    setForm((prev) =>
      patchSelectionFields(prev, {
        shaft_grade: shaftGrades[0],
      }),
    );
  }, [shaftGrade, shaftGrades, setForm]);

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
        options,
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
      applyToleranceGradeChange(
        prev,
        feature,
        value,
      ),
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

    holeLetter,
    holeGrade,
    shaftLetter,
    shaftGrade,

    holeLetterOptions,
    holeGradeOptions,
    shaftLetterOptions,
    shaftGradeOptions,

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