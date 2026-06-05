// features/finishing/ui/execution/ExecutionTableRow.tsx

import { Table } from "@shared/ui/table/Table/Table";
import { RefObject } from "react";
import { formatNumber } from "@shared/lib/format/formatNumber";

import type { ExecutionStep } from "@shared/execution";
import {
  isStepEditableCompleted,
  isStepInputEditable,
  getStepMeasurementValue,
} from "@shared/execution/executionState";

import { ExecutionRowActions } from "./ExecutionRowActions";
import { ExecutionDisplay } from "../ExecutionField/ExecutionDisplay";
import { ExecutionInput } from "../ExecutionField/ExecutionInput";
import { ExecutionValue } from "../ExecutionField/ExecutionValue";

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
     Derived state
  ============================================================ */

  const isEditing = editingStep === step.index;

  const isActive = step.status === "active";

  const isEditableCompleted =
    isStepEditableCompleted(step, finished);

  const isInputEditable =
    isStepInputEditable(step, finished, isEditing);

  const deltaValue =
    cutMode === "deltaD"
      ? step.data.deltaD
      : step.data.deltaD * 0.5;

  const measurementValue =
    getStepMeasurementValue(step);

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

  const isPending = step.status === "pending";

  /* =========================
     Ref registration
  ========================= */

  const registerRef = (el: HTMLInputElement | null) => {
    const refs = inputRefs.current;
    if (!refs) return;

    if (el) {
      refs[step.index] = el;
    } else {
      delete refs[step.index];
    }
  };

   /* ============================================================
     Cells
  ============================================================ */

  const stepCell = (
  <ExecutionDisplay>
    {step.index}
  </ExecutionDisplay>
);
  

  const startDiameterCell = isPending ? null : (
    <ExecutionValue
      value={formatNumber(step.data.startDiameter, decimals)}
    />
  );

  const deltaCell = isPending ? null : (
    <ExecutionValue
      value={formatNumber(deltaValue, decimals)}
    />
  );

  const measurementCell = isPending
    ? null
    : isInputEditable
    ? (
        <ExecutionInput
          ref={registerRef}
          value={displayValue}
          placeholder={placeholder}
          error={error}
          onChange={(v) => onDraftChange(step.index, v)}
          onSubmit={() => onConfirm(step.index)}
        />
      )
    : (
        <ExecutionValue value={measurementValue} />
      );

  const actionsCell = (
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
  );


  /* ============================================================
     Render
  ============================================================ */

  return (
    <Table.Row isActive={isActive}>

      <Table.Cell>
        {stepCell}
      </Table.Cell>

      <Table.Cell align="right">
        {startDiameterCell}
      </Table.Cell>

      <Table.Cell align="right">
        {deltaCell}
      </Table.Cell>

      <Table.Cell align="right">
        {measurementCell}
      </Table.Cell>

      <Table.Cell align="center">
        {actionsCell}
      </Table.Cell>

    </Table.Row>
  );
}
