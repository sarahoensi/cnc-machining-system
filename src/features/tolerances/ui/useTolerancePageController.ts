// features/tolerances/ui/useTolerancePageController.ts

import { useEffect, useMemo } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { useSavedResults } from "@shared/savedResults";
import { getTauriCommandError } from "@shared/api/tauriError";
import {
  clearMachineFields,
  handleCalculateAsync,
  handleModeChange,
  handleUserEdit,
} from "@shared/form/engine/formEngine";
import { machineField, userField } from "@shared/form/types/fields";

import {
  listIso286ToleranceOptionsApi,
} from "../api/client";
import { solveTolerance } from "../api/solveTolerance";
import type {
  ToleranceMode,
  ToleranceObjectType,
} from "../api/types";

import {
  createInitialToleranceForm,
  migrateToleranceForm,
  ToleranceFormState,
  ToleranceKey,
} from "../domain/toleranceForm";
import { parseTolerance } from "../domain/parseTolerance";
import { validateToleranceForm } from "../domain/validateToleranceForm";

import {
  gradesForZone,
  preserveEquivalentToleranceSelection,
  reconcileSelectionFields,
} from "../domain/toleranceOptions";


const validInputSets: readonly (readonly ToleranceKey[])[] = [
  ["nominal", "hole_letter", "hole_grade", "shaft_letter", "shaft_grade"],
];
const mutuallyExclusivePairs: readonly (readonly [ToleranceKey, ToleranceKey])[] =
  [];
const resultKeys = ["upper_um", "lower_um", "min_mm", "max_mm"] as const;

export function useTolerancePageController() {
  const [storedForm, setForm] = useFeatureForm(
    "tolerances",
    createInitialToleranceForm,
  );
  const form = useMemo(() => migrateToleranceForm(storedForm), [storedForm]);
  const savedResults = useSavedResults<ToleranceFormState>({
    storageKey: "tolerances-history",
    normalizeLoadedForm: (loadedForm) => {
      const migrated = migrateToleranceForm(loadedForm);

      return {
        ...migrated,
        extras: {
          ...migrated.extras,
          options: form.extras.options,
          loadingOptions: form.extras.loadingOptions,
        },
      };
    },
  });

  useEffect(() => {
    if (form !== storedForm) {
      setForm(form);
    }
  }, [form, setForm, storedForm]);

  const { options } = form.extras;
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
        if (!cancelled) {
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
    }

    void loadOptions();

    return () => {
      cancelled = true;
    };
  }, [setForm]);

  useEffect(() => {
    if (holeGrades.length === 0 || holeGrades.includes(holeGrade)) return;

    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: {
        ...clearResultFields(prev),
        hole_grade: userField(holeGrades[0]),
      },
      formError: undefined,
    }));
  }, [holeGrade, holeGrades, setForm]);

  useEffect(() => {
    if (shaftGrades.length === 0 || shaftGrades.includes(shaftGrade)) return;

    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: {
        ...clearResultFields(prev),
        shaft_grade: userField(shaftGrades[0]),
      },
      formError: undefined,
    }));
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
    setForm((prev) => {
      const next = handleUserEdit(
        prev,
        key,
        value,
        validInputSets,
        mutuallyExclusivePairs,
      );

      return next;
    });
  }

  function onToleranceLetterChange(
    feature: ToleranceObjectType,
    value: string,
  ) {
    const nextGrades =
      feature === "hole"
        ? gradesForZone(options.holes, value)
        : gradesForZone(options.shafts, value);

    updateEditingFields(
      feature === "hole"
        ? {
            hole_letter: value,
            hole_grade: nextGrades[0] ?? "",
          }
        : {
            shaft_letter: value,
            shaft_grade: nextGrades[0] ?? "",
          },
    );
  }

  function onToleranceGradeChange(
    feature: ToleranceObjectType,
    value: string,
  ) {
    updateEditingFields(
      feature === "hole"
        ? { hole_grade: value }
        : { shaft_grade: value },
    );
  }

  async function calculate() {
    const next = await handleCalculateAsync(
      form,
      parseTolerance,
      solveTolerance,
      validateToleranceForm,
    );

    if (next.status === "solved") {
      const fields = { ...next.fields };

      for (const key of resultKeys) {
        const resultField = fields[key];
        const hasMachineValue =
          resultField.machineValue != null || resultField.value.trim() !== "";

        if (!hasMachineValue) continue;

        fields[key] = machineField(
          resultField.machineValue != null
            ? String(resultField.machineValue)
            : resultField.value,
          {
            ...resultField,
            source: "machine",
          },
        );
      }

      setForm({
        ...next,
        fields,
      });
      return next;
    }

    setForm(next);
    return next;
  }

  function resetForm() {
    setForm((prev) => {
      const initial = createInitialToleranceForm();
      const extras = {
        ...initial.extras,
        options: prev.extras.options,
        loadingOptions: prev.extras.loadingOptions,
      };

      return {
        ...initial,
        fields: reconcileSelectionFields(initial.fields, extras.options),
        extras,
      };
    });
  }

  function save() {
    savedResults.save(form);
  }

  function load(entry: (typeof savedResults.history)[number]) {
    setForm(savedResults.load(entry));
  }

  function updateEditingFields(
    patch: Partial<Record<ToleranceKey, string>>,
  ) {
    setForm((prev) => {
      const fields = clearResultFields(prev);

      for (const key in patch) {
        const typedKey = key as ToleranceKey;
        fields[typedKey] = userField(patch[typedKey] ?? "");
      }

      return {
        ...prev,
        status: "editing",
        fields,
        formError: undefined,
      };
    });
  }

  return {
    form,
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


function clearResultFields(
  form: ToleranceFormState,
): ToleranceFormState["fields"] {
  const fields = clearMachineFields(form.fields);

  return {
    ...fields,
    nominal: {
      ...fields.nominal,
      invalid: false,
      error: undefined,
    },
  };
}

function getToleranceErrorMessage(error: unknown) {
  const commandError = getTauriCommandError(error);
  if (commandError?.message) return commandError.message;
  if (typeof error === "string") return error;
  if (error instanceof Error && error.message) return error.message;
  return "ISO 286 calculation failed";
}
