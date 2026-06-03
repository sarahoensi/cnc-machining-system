import { useEffect, useMemo, useState } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { getTauriCommandError } from "@shared/api/tauriError";
import { machineField, userField } from "@shared/form/types/fields";

import {
  listIso286ToleranceOptionsApi,
  lookupIso286ToleranceApi,
} from "../api/client";
import type {
  Iso286MemberResult,
  ToleranceMode,
  ToleranceObjectType,
  ToleranceOption,
} from "../api/types";
import { buildLookupIso286ToleranceRequest } from "../domain/buildRequest";
import {
  buildToleranceFormInput,
  createInitialToleranceForm,
  resultField,
  ToleranceFormState,
} from "../domain/toleranceForm";
import { validateToleranceForm } from "../domain/validateToleranceForm";

export function useTolerancePageController() {
  const [form, setForm] = useFeatureForm(
    "tolerances",
    createInitialToleranceForm,
  );
  const [tableOpen, setTableOpen] = useState(false);

  const { holeLetter, holeGrade, shaftLetter, shaftGrade, options } =
    form.extras;

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
          extras: reconcileSelections({
            ...prev.extras,
            options: response,
            loadingOptions: false,
          }),
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
      fields: clearResultFields(prev),
      extras: {
        ...prev.extras,
        holeGrade: holeGrades[0],
        resultCode: undefined,
      },
      formError: undefined,
    }));
  }, [holeGrade, holeGrades, setForm]);

  useEffect(() => {
    if (shaftGrades.length === 0 || shaftGrades.includes(shaftGrade)) return;

    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: clearResultFields(prev),
      extras: {
        ...prev.extras,
        shaftGrade: shaftGrades[0],
        resultCode: undefined,
      },
      formError: undefined,
    }));
  }, [shaftGrade, shaftGrades, setForm]);

  function onModeChange(value: ToleranceMode) {
    updateEditingForm({
      mode: value,
      resultCode: undefined,
    });
  }

  function onNominalChange(value: string) {
    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: {
        ...clearResultFields(prev),
        nominal: userField(value),
      },
      extras: {
        ...prev.extras,
        resultCode: undefined,
      },
      formError: undefined,
    }));
    setTableOpen(false);
  }

  function onToleranceLetterChange(
    feature: ToleranceObjectType,
    value: string,
  ) {
    const nextGrades =
      feature === "hole"
        ? gradesForZone(options.holes, value)
        : gradesForZone(options.shafts, value);

    updateEditingForm(
      feature === "hole"
        ? {
            holeLetter: value,
            holeGrade: nextGrades[0] ?? "",
            resultCode: undefined,
          }
        : {
            shaftLetter: value,
            shaftGrade: nextGrades[0] ?? "",
            resultCode: undefined,
          },
    );
  }

  function onToleranceGradeChange(
    feature: ToleranceObjectType,
    value: string,
  ) {
    updateEditingForm(
      feature === "hole"
        ? { holeGrade: value, resultCode: undefined }
        : { shaftGrade: value, resultCode: undefined },
    );
  }

  async function calculate() {
    const input = buildToleranceFormInput(form);
    const errors = validateToleranceForm(input);
    const errorMessages = toleranceFormErrors(errors);

    setForm((prev) => ({
      ...prev,
      fields: {
        ...prev.fields,
        nominal: {
          ...prev.fields.nominal,
          invalid: Boolean(errors.nominal),
          error: errors.nominal,
        },
      },
      formError: errorMessages.length > 0 ? errorMessages : undefined,
    }));

    if (Object.keys(errors).length > 0) return errors;

    try {
      const response = await lookupIso286ToleranceApi(
        buildLookupIso286ToleranceRequest(input),
      );

      setForm((prev) => ({
        status: "solved",
        fields: {
          ...prev.fields,
          nominal: {
            ...prev.fields.nominal,
            invalid: false,
            error: undefined,
          },
          upper_um: toleranceMachineField(response.upper_um),
          lower_um: toleranceMachineField(response.lower_um),
          min_mm: toleranceMachineField(response.min_mm),
          max_mm: toleranceMachineField(response.max_mm),
        },
        extras: {
          ...prev.extras,
          resultCode: response.code,
        },
        formError: undefined,
      }));
    } catch (error) {
      setForm((prev) => ({
        ...prev,
        status: "editing",
        fields: clearResultFields(prev),
        extras: {
          ...prev.extras,
          resultCode: undefined,
        },
        formError: getToleranceErrorMessage(error),
      }));
    }

    return {};
  }

  function resetForm() {
    setForm((prev) => {
      const initial = createInitialToleranceForm();
      const extras = reconcileSelections({
        ...initial.extras,
        options: prev.extras.options,
        loadingOptions: prev.extras.loadingOptions,
      });

      return {
        ...initial,
        extras,
      };
    });
    setTableOpen(false);
  }

  function updateEditingForm(patch: Partial<ToleranceFormState["extras"]>) {
    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: clearResultFields(prev),
      extras: {
        ...prev.extras,
        ...patch,
      },
      formError: undefined,
    }));
    setTableOpen(false);
  }

  return {
    form,
    result: resultFromForm(form),
    tableOpen,
    setTableOpen,
    onModeChange,
    onNominalChange,
    onToleranceLetterChange,
    onToleranceGradeChange,
    calculate,
    resetForm,
  };
}

function reconcileSelections(
  extras: ToleranceFormState["extras"],
): ToleranceFormState["extras"] {
  const hole = validSelection(
    extras.options.holes,
    extras.holeLetter,
    extras.holeGrade,
    "H",
    "7",
  );
  const shaft = validSelection(
    extras.options.shafts,
    extras.shaftLetter,
    extras.shaftGrade,
    "g",
    "6",
  );

  return {
    ...extras,
    holeLetter: hole.zone,
    holeGrade: hole.grade,
    shaftLetter: shaft.zone,
    shaftGrade: shaft.grade,
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

function clearResultFields(form: ToleranceFormState) {
  return {
    ...form.fields,
    nominal: {
      ...form.fields.nominal,
      invalid: false,
      error: undefined,
    },
    upper_um: resultField(),
    lower_um: resultField(),
    min_mm: resultField(),
    max_mm: resultField(),
  };
}

function toleranceMachineField(value: number) {
  return machineField(String(value), {
    locked: true,
    machineValue: value,
  });
}

function resultFromForm(form: ToleranceFormState): Iso286MemberResult | null {
  const upper = form.fields.upper_um.machineValue;
  const lower = form.fields.lower_um.machineValue;
  const min = form.fields.min_mm.machineValue;
  const max = form.fields.max_mm.machineValue;

  if (
    upper == null ||
    lower == null ||
    min == null ||
    max == null ||
    !form.extras.resultCode
  ) {
    return null;
  }

  return {
    code: form.extras.resultCode,
    zone: "",
    grade: Number.NaN,
    upper_um: upper,
    lower_um: lower,
    min_mm: min,
    max_mm: max,
    source_table: null,
    source_file: null,
  };
}

function toleranceFormErrors(
  errors: Partial<Record<keyof ReturnType<typeof buildToleranceFormInput>, string>>,
) {
  return Object.entries(errors)
    .filter(([key]) => key !== "nominal")
    .map(([, message]) => message)
    .filter((message): message is string => Boolean(message));
}

function getToleranceErrorMessage(error: unknown) {
  const commandError = getTauriCommandError(error);
  if (commandError?.message) return commandError.message;
  if (typeof error === "string") return error;
  if (error instanceof Error && error.message) return error.message;
  return "ISO 286 calculation failed";
}
