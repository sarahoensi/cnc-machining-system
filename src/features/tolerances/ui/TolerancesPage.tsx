import { useEffect, useMemo, useState } from "react";

import { useFeatureForm } from "@app/providers/FormStateProvider";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { usePageTitle } from "@app/providers/TitleContextProvider";
import { getTauriCommandError } from "@shared/api/tauriError";
import { machineField, userField } from "@shared/form/types/fields";
import { FormActions } from "@shared/ui/components/form/FormActions/FormActions";
import { FormError } from "@shared/ui/components/form/FormError/FormError";
import { FormModeField } from "@shared/ui/components/form/fields/FormModeField";
import { FormNumberField } from "@shared/ui/components/form/fields/FormNumberField";
import { FormSelectMenuField } from "@shared/ui/components/form/fields/FormSelectMenuField";
import { Modal, ModalScrollArea } from "@shared/ui/components/overlay/Modal/Modal";
import { Table } from "@shared/ui/components/table/Table";
import { formatNumber } from "@shared/ui/format/formatNumber";
import { FormLayout } from "@shared/ui/layout/container/FormLayout/FormLayout";
import { FormSection } from "@shared/ui/layout/container/FormSection/FormSection";
import { Button } from "@shared/ui/primitives/Button/Button";
import { useFormNavigation } from "@shared/ui";

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

import "./TolerancesPage.css";

const modeOptions = [
  { value: "hole", label: "Hole" },
  { value: "shaft", label: "Shaft" },
] as const;

export function TolerancesPage() {
  usePageTitle("Tolerances");

  const { decimals } = useDisplaySettings();
  const [form, setForm] = useFeatureForm(
    "tolerances",
    createInitialToleranceForm,
  );
  const [tableOpen, setTableOpen] = useState(false);
  const navigation = useFormNavigation({
    keys: ["nominal"] as const,
    autoFocusOnMount: true,
    activePath: "/tolerances",
    onSubmit: onCalculate,
  });

  const {
    mode,
    holeLetter,
    holeGrade,
    shaftLetter,
    shaftGrade,
    options,
    loadingOptions,
  } = form.extras;

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
  }, []);

  useEffect(() => {
    if (holeGrades.length > 0 && !holeGrades.includes(holeGrade)) {
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
    }
  }, [holeGrade, holeGrades, setForm]);

  useEffect(() => {
    if (shaftGrades.length > 0 && !shaftGrades.includes(shaftGrade)) {
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
    }
  }, [shaftGrade, shaftGrades, setForm]);

  function onModeChange(value: ToleranceMode) {
    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: clearResultFields(prev),
      extras: {
        ...prev.extras,
        mode: value,
        resultCode: undefined,
      },
      formError: undefined,
    }));
    setTableOpen(false);
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

  function onToleranceClassChange(
    patch: Partial<
      Pick<
        ToleranceFormState["extras"],
        "holeLetter" | "holeGrade" | "shaftLetter" | "shaftGrade"
      >
    >,
  ) {
    setForm((prev) => ({
      ...prev,
      status: "editing",
      fields: clearResultFields(prev),
      extras: {
        ...prev.extras,
        ...patch,
        resultCode: undefined,
      },
      formError: undefined,
    }));
    setTableOpen(false);
  }

  async function onCalculate() {
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

    if (Object.keys(errors).length > 0) {
      navigation.focusFirstInvalidAfterRender((key) =>
        Boolean(errors[key]),
      );
      return;
    }

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
  }

  function onReset() {
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
    navigation.focusFirstAfterRender();
  }

  const result = resultFromForm(form);
  const canOpenTable = Boolean(result);

  const inputFields = (
    <FormSection>
      <FormModeField
        label="Mode"
        value={mode}
        options={modeOptions}
        onChange={onModeChange}
      />

      <FormNumberField
        label="Nominal size"
        unit="mm"
        field={form.fields.nominal}
        onChange={onNominalChange}
        ref={navigation.register("nominal")}
        onKeyDown={navigation.handleKeyDown("nominal")}
      />

      {mode === "hole" && (
        <ToleranceClassFields
          feature="hole"
          options={options.holes}
          letter={holeLetter}
          grade={holeGrade}
          disabled={loadingOptions}
          onLetterChange={(value) => {
            const nextGrades = gradesForZone(options.holes, value);
            onToleranceClassChange({
              holeLetter: value,
              holeGrade: nextGrades[0] ?? "",
            });
          }}
          onGradeChange={(value) => {
            onToleranceClassChange({ holeGrade: value });
          }}
        />
      )}

      {mode === "shaft" && (
        <ToleranceClassFields
          feature="shaft"
          options={options.shafts}
          letter={shaftLetter}
          grade={shaftGrade}
          disabled={loadingOptions}
          onLetterChange={(value) => {
            const nextGrades = gradesForZone(options.shafts, value);
            onToleranceClassChange({
              shaftLetter: value,
              shaftGrade: nextGrades[0] ?? "",
            });
          }}
          onGradeChange={(value) => {
            onToleranceClassChange({ shaftGrade: value });
          }}
        />
      )}
    </FormSection>
  );

  const error = form.formError ? <FormError error={form.formError} /> : null;
  const actions = <FormActions onCalculate={onCalculate} onReset={onReset} />;

  return (
    <>
      <div className="tolerances-page">
        <div className="tolerances-input-column">
          <div ref={navigation.containerRef}>
            <FormLayout fields={inputFields} error={error} actions={actions} />
          </div>
        </div>

        <div className="tolerances-output-column">
          <ResultFields form={form} />

          <div className="tolerances-result-actions">
            <Button
              variant="secondary"
              size="small"
              onClick={() => setTableOpen(true)}
              disabled={!canOpenTable}
            >
              View in table
            </Button>
          </div>
        </div>
      </div>

      {tableOpen && result && (
        <Modal
          title="ISO 286 result table"
          onClose={() => setTableOpen(false)}
          size="lg"
          height="auto"
        >
          <ModalScrollArea>
            <SingleResultTable result={result} decimals={decimals} />
          </ModalScrollArea>
        </Modal>
      )}
    </>
  );
}

function ToleranceClassFields({
  feature,
  options,
  letter,
  grade,
  disabled,
  onLetterChange,
  onGradeChange,
}: {
  feature: ToleranceObjectType;
  options: ToleranceOption[];
  letter: string;
  grade: string;
  disabled: boolean;
  onLetterChange: (value: string) => void;
  onGradeChange: (value: string) => void;
}) {
  const grades = gradesForZone(options, letter);
  const title = feature === "hole" ? "Hole" : "Shaft";

  return (
    <>
      <FormSelectMenuField
        label={`${title} class`}
        valueLabel={letter || "-"}
        options={options.map((option) => ({
          value: option.zone,
          label: option.zone,
        }))}
        onSelect={onLetterChange}
        disabled={disabled}
      />

      <FormSelectMenuField
        label={`${title} grade`}
        valueLabel={grade || "-"}
        options={grades.map((value) => ({
          value,
          label: value,
        }))}
        onSelect={onGradeChange}
        disabled={disabled || grades.length === 0}
      />
    </>
  );
}

function ResultFields({ form }: { form: ToleranceFormState }) {
  return (
    <FormSection variant="result">
      <FormNumberField
        label="Upper"
        unit="um"
        field={form.fields.upper_um}
        readonly
        onChange={() => undefined}
      />
      <FormNumberField
        label="Lower"
        unit="um"
        field={form.fields.lower_um}
        readonly
        onChange={() => undefined}
      />
      <FormNumberField
        label="Minimum"
        unit="mm"
        field={form.fields.min_mm}
        readonly
        onChange={() => undefined}
      />
      <FormNumberField
        label="Maximum"
        unit="mm"
        field={form.fields.max_mm}
        readonly
        onChange={() => undefined}
      />
    </FormSection>
  );
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
    grade: fallback?.grades[0] != null ? String(fallback.grades[0]) : currentGrade,
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

function SingleResultTable({
  result,
  decimals,
}: {
  result: Iso286MemberResult;
  decimals: number;
}) {
  return (
    <Table.Root className="tolerances-table">
      <Table.Head>
        <Table.HeadRow>
          <Table.HeaderCell>Code</Table.HeaderCell>
          <Table.HeaderCell align="right">Upper</Table.HeaderCell>
          <Table.HeaderCell align="right">Lower</Table.HeaderCell>
          <Table.HeaderCell align="right">Minimum</Table.HeaderCell>
          <Table.HeaderCell align="right">Maximum</Table.HeaderCell>
        </Table.HeadRow>
      </Table.Head>
      <Table.Body>
        <Table.BodyRow>
          <Table.Cell>{result.code}</Table.Cell>
          <Table.Cell align="right">{result.upper_um} um</Table.Cell>
          <Table.Cell align="right">{result.lower_um} um</Table.Cell>
          <Table.Cell align="right">
            {formatNumber(result.min_mm, decimals)} mm
          </Table.Cell>
          <Table.Cell align="right">
            {formatNumber(result.max_mm, decimals)} mm
          </Table.Cell>
        </Table.BodyRow>
      </Table.Body>
    </Table.Root>
  );
}
