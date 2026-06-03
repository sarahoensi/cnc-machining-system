import { useEffect, useMemo } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
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
  ToleranceOption,
} from "../api/types";
import {
  createInitialToleranceForm,
  migrateToleranceForm,
  ToleranceFormState,
  ToleranceKey,
} from "../domain/toleranceForm";
import { parseTolerance } from "../domain/parseTolerance";
import { validateToleranceForm } from "../domain/validateToleranceForm";

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
    setForm((prev) =>
      handleModeChange(prev, {
        ...prev.extras,
        mode: value,
      }),
    );
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
  };
}

function reconcileSelectionFields(
  fields: ToleranceFormState["fields"],
  options: ToleranceFormState["extras"]["options"],
): ToleranceFormState["fields"] {
  const hole = validSelection(
    options.holes,
    fields.hole_letter.value,
    fields.hole_grade.value,
    "H",
    "7",
  );
  const shaft = validSelection(
    options.shafts,
    fields.shaft_letter.value,
    fields.shaft_grade.value,
    "g",
    "6",
  );

  return {
    ...fields,
    hole_letter: userField(hole.zone),
    hole_grade: userField(hole.grade),
    shaft_letter: userField(shaft.zone),
    shaft_grade: userField(shaft.grade),
  };
}

function validSelection(
  options: ToleranceOption[],
  currentZone: string,
  currentGrade: string,
  preferredZone: string,
  preferredGrade: string,
) {
  const current = options.find((row) => row.zone === currentZone);
  if (current?.grades.includes(Number(currentGrade))) {
    return { zone: currentZone, grade: currentGrade };
  }

  const preferred = options.find((row) => row.zone === preferredZone);
  if (preferred) {
    return {
      zone: preferred.zone,
      grade: preferred.grades.includes(Number(preferredGrade))
        ? preferredGrade
        : String(preferred.grades[0] ?? ""),
    };
  }

  const fallback = options[0];
  return {
    zone: fallback?.zone ?? currentZone,
    grade:
      fallback?.grades[0] != null ? String(fallback.grades[0]) : currentGrade,
  };
}

function gradesForZone(options: ToleranceOption[], zone: string) {
  return (
    options.find((option) => option.zone === zone)?.grades.map(String) ?? []
  );
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
