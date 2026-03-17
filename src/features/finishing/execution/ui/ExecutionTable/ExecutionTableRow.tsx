// features/finishing/ui/execution/ExecutionTableRow.tsx

import { Table } from "@shared/ui/components/table/Table/Table";
import { ExecutionInput } from "@shared/ui/components/execution/ExecutionInput";
import { ExecutionValue } from "@shared/ui/components/execution/ExecutionValue";
import { RefObject } from "react";
import { formatNumber } from "@shared/ui/format/formatNumber";

import type { ExecutionStep } from "@shared/execution";
import { ExecutionRowActions } from "./ExecutionRowActions";

type CutMode = "deltaD" | "ae";

type FinishingRowData = {
  startDiameter: number;
  deltaD: number;
  expectedDiameter: number;
};

type Props = {
  step: ExecutionStep<FinishingRowData>;
  cutMode: CutMode;
  decimals: number;

  draft?: string;
  error?: string;

  editingStep: number | null;

  inputRefs: RefObject<
    Record<number, HTMLInputElement | null>
  >;

  onDraftChange(step: number, value: string): void;
  onConfirm(step: number): void;

  onStartEdit(step: number, value: string): void;
  onCancelEdit(): void;

  finished: boolean
};

export function ExecutionTableRow({
  step,
  cutMode,
  decimals,
  draft,
  error,
  editingStep,
  inputRefs,
  finished,
  onDraftChange,
  onConfirm,
  onStartEdit,
  onCancelEdit,
}: Props) {

  /* ============================================================
     Row state
  ============================================================ */

  const isEditing = editingStep === step.index;
  const isActive = step.status === "active";

  const isEditableCompleted =
    !finished &&
    step.status === "completed" &&
    step.editable;

  const isInputEditable =
    isEditing || (isActive && !finished);

  /* ============================================================
     Display values
  ============================================================ */

  const deltaValue =
    cutMode === "deltaD"
      ? step.data.deltaD
      : step.data.deltaD * 0.5;

  const measurementValue =
    step.measurement.value ?? "";

  const displayValue = isInputEditable
    ? draft !== undefined
      ? draft
      : measurementValue
    : measurementValue;

  const placeholder =
    measurementValue
      ? undefined
      : formatNumber(
        step.data.expectedDiameter,
        decimals
      );

  /* ============================================================
     Render
  ============================================================ */

  return (
    <Table.Row isActive={isActive}>

      {/* Step index */}

      <Table.Cell>
        {step.index}
      </Table.Cell>

      {/* Start diameter */}

      <Table.Cell align="right">
        {step.status === "pending" ? null : (
          <ExecutionValue
            value={formatNumber(step.data.startDiameter, decimals)}
          />
        )}
      </Table.Cell>

      {/* ΔD / ae */}

      <Table.Cell align="right">
        {step.status === "pending" ? null : (
          <ExecutionValue
            value={formatNumber(deltaValue, decimals)}
          />
        )}
      </Table.Cell>

      {/* Measurement */}

      <Table.Cell align="right">
        {step.status === "pending" ? null : isEditing ? (
          <ExecutionInput
            ref={(el) => {
              const refs = inputRefs.current;
              if (!refs) return;

              if (el) {
                refs[step.index] = el;
              } else {
                delete refs[step.index];
              }
            }}
            value={displayValue}
            placeholder={placeholder}
            error={error}
            onChange={(v) => onDraftChange(step.index, v)}
            onSubmit={() => onConfirm(step.index)}
          />
        ) : isActive ? (
          <ExecutionInput
            value={displayValue}
            placeholder={placeholder}
            onChange={(v) => onDraftChange(step.index, v)}
            onSubmit={() => onConfirm(step.index)}
          />
        ) : (
          <ExecutionValue value={measurementValue} />
        )}
      </Table.Cell>

      {/* Actions */}

      <Table.Cell align="center">

        <ExecutionRowActions
          stepIndex={step.index}
          isEditing={isEditing}
          isActive={isActive}
          isEditableCompleted={isEditableCompleted}
          value={displayValue}
          measurementValue={measurementValue}
          onConfirm={onConfirm}
          onStartEdit={onStartEdit}
          onCancelEdit={onCancelEdit}
          finished={finished}
        />

      </Table.Cell>

    </Table.Row>
  );
}