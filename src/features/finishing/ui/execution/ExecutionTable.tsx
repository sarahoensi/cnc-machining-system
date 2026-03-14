// features/finishing/ui/execution/ExecutionTable.tsx

import { useState, useRef, useEffect } from "react";

import { Table } from "@shared/ui/components/table/Table/Table";
import { TableHeaderSelect } from "@shared/ui/components/table/Table";

import { ExecutionNumberField } from "@shared/ui/components/execution/ExecutionNumberField";

import {
  CancelButton,
  OkButton,
  RegisterButton,
  EditButton,
} from "@shared/ui/components/primitives/Button/Button";

import type { ExecutionState } from "@shared/execution";

import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { formatNumber } from "@shared/ui/format/formatNumber";
import { getTauriCommandError } from "@shared/api/tauriError";

import "./ExecutionTable.css";

import { parseDecimalInput } from "@shared/parsing/decimalParser";



/* ============================================================
   Step data
============================================================ */

type FinishingStepData = {
  startDiameter: number;
  deltaD: number;
  expectedDiameter: number;
};

/* ============================================================
   Props
============================================================ */

type Props = {
  execution: ExecutionState<FinishingStepData>;
  onRegisterMeasurement(
    step: number,
    measurement: number
  ): Promise<void>;
};

/* ============================================================
   Component
============================================================ */

export function FinishingExecutionTable({
  execution,
  onRegisterMeasurement,
}: Props) {

  const { decimals } = useDisplaySettings();

  const [cutMode, setCutMode] =
    useState<"deltaD" | "ae">("deltaD");

  const [editingStep, setEditingStep] =
    useState<number | null>(null);

  const [drafts, setDrafts] =
    useState<Record<number, string>>({});

  const [errors, setErrors] =
    useState<Record<number, string>>({});

  /* ============================================================
     Input refs (for autofocus)
  ============================================================ */

  const inputRefs =
    useRef<Record<number, HTMLInputElement | null>>({});

  useEffect(() => {

    if (editingStep !== null) return;

    const active = execution.steps.find(
      s => s.status === "active"
    );

    if (!active) return;

    const input = inputRefs.current[active.index];

    input?.focus();

  }, [execution, editingStep]);

  useEffect(() => {
    if (editingStep === null) return;

    const input = inputRefs.current[editingStep];

    input?.focus();
  }, [editingStep]);


  /* ----------------------------------------------------------
     Draft helpers
  ---------------------------------------------------------- */

  function updateDraft(step: number, value: string) {
    setDrafts(d => ({
      ...d,
      [step]: value,
    }));
  }

  function startEdit(step: number, value: string) {
    setDrafts(d => ({
      ...d,
      [step]: value ?? "",
    }));
    setEditingStep(step);
  }

  function cancelEdit() {
    setEditingStep(null);
  }

  async function confirmEdit(step: number) {

    const value = drafts[step];

    if (!value) return;

    const { normalized, number } = parseDecimalInput(value);

    if (number === null) {
      setErrors(e => ({
        ...e,
        [step]: "Invalid number"
      }));
      return;
    }

    try {

      await onRegisterMeasurement(step, number);

      setDrafts(d => ({
        ...d,
        [step]: normalized
      }));

      setEditingStep(null);

    } catch (error) {

      const te = getTauriCommandError(error);
      const firstError = te?.fieldErrors?.[0];

      if (!firstError) return;

      setErrors(e => ({
        ...e,
        [step]: firstError.message
      }));
    }
  }

  /* ----------------------------------------------------------
     Render
  ---------------------------------------------------------- */

  return (

    <Table.Root className="execution-table">

      {/* ===================================================== */}
      {/* Header                                                */}
      {/* ===================================================== */}

      <Table.Head>

        <Table.Row>

          <Table.HeaderCell>
            Step
          </Table.HeaderCell>

          <Table.HeaderCell align="center">
            Start Ø
          </Table.HeaderCell>

          <TableHeaderSelect
            value={cutMode}
            onChange={setCutMode}
            options={[
              { value: "deltaD", label: "ΔD" },
              { value: "ae", label: "ae" },
            ]}
            align="center"
          />

          <Table.HeaderCell align="center">
            Measurement
          </Table.HeaderCell>

          <Table.HeaderCell align="center" />

        </Table.Row>

      </Table.Head>

      {/* ===================================================== */}
      {/* Body                                                  */}
      {/* ===================================================== */}

      <Table.Body>

        {execution.steps.map(step => {

          const draft =
            drafts[step.index] ??
            step.measurement.value ??
            "";

          const deltaValue =
            cutMode === "deltaD"
              ? step.data.deltaD
              : step.data.deltaD / 2;

          const isEditing =
            editingStep === step.index;

          const isActive =
            step.status === "active";

          const isEditableCompleted =
            step.status === "completed" &&
            step.editable;

          return (

            <Table.Row
              key={step.index}
              isActive={isActive}
            >

              {/* Step index */}

              <Table.Cell>
                {step.index}
              </Table.Cell>

              {/* Start diameter */}

              <Table.Cell align="right">

                <ExecutionNumberField
                  state={step.status}
                  value={formatNumber(
                    step.data.startDiameter,
                    decimals
                  )}
                  readonly
                />

              </Table.Cell>

              {/* ΔD / ae */}

              <Table.Cell align="right">

                <ExecutionNumberField
                  state={step.status}
                  value={formatNumber(
                    deltaValue,
                    decimals
                  )}
                  readonly
                />

              </Table.Cell>

              {/* Measurement */}

              <Table.Cell align="right">

                <ExecutionNumberField
                  ref={(el) => {
                    if (el) {
                      inputRefs.current[step.index] = el;
                    } else {
                      delete inputRefs.current[step.index];
                    }
                  }}
                  state={isEditing ? "active" : step.status}

                  value={
                    isEditing || isActive
                      ? draft
                      : step.measurement.value
                  }

                  placeholder={
                    step.measurement.value
                      ? undefined
                      : formatNumber(
                        step.data.expectedDiameter,
                        decimals
                      )
                  }

                  error={errors[step.index]}

                  onChange={
                    isEditing || isActive
                      ? (v) => {
                        updateDraft(step.index, v);

                        setErrors(e => {
                          const next = { ...e };
                          delete next[step.index];
                          return next;
                        });
                      }
                      : undefined
                  }

                  onSubmit={() =>
                    confirmEdit(step.index)
                  }
                />

              </Table.Cell>

              {/* Action column */}

              <Table.Cell align="center">

                {/* EDIT MODE */}

                {isEditing && (

                  <>
                    <OkButton
                      onClick={() =>
                        confirmEdit(
                          step.index
                        )
                      }
                    />

                    <CancelButton
                      onClick={cancelEdit}
                    />
                  </>

                )}

                {/* ACTIVE STEP */}

                {!isEditing && isActive && (

                  <RegisterButton
                    disabled={!draft}
                    onClick={() =>
                      confirmEdit(
                        step.index
                      )
                    }
                  />

                )}

                {/* COMPLETED STEP */}

                {!isEditing &&
                  isEditableCompleted && (

                    <EditButton
                      onClick={() =>
                        startEdit(
                          step.index,
                          step.measurement.value
                        )
                      }
                    >
                      Edit
                    </EditButton>

                  )}

              </Table.Cell>

            </Table.Row>

          );

        })}

      </Table.Body>

    </Table.Root>
  );
}