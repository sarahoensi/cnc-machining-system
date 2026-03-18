// features/finishing/ui/execution/ExecutionTable.tsx

import { useState, useRef } from "react";

import {useExecutionFocus} from "./useExecutionFocus";

import { Table } from "@shared/ui/components/table/Table/Table";
import { TableHeaderSelect } from "@shared/ui/components/table/Table";

import type { ExecutionState } from "@shared/execution";

import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";

import { ExecutionTableRow } from "./ExecutionTableRow";
import { useExecutionEditing } from "./useExecutionEditing";

import "./ExecutionTable.css";

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
  onRegisterMeasurement
}: Props) {

  const { decimals } = useDisplaySettings();
  const finished = execution.finished;

  const [cutMode, setCutMode] =
    useState<"deltaD" | "ae">("deltaD");

  const {
    editingStep,
    drafts,
    errors,
    updateDraft,
    startEdit,
    cancelEdit,
    confirmEdit,
  } = useExecutionEditing(onRegisterMeasurement);

  

  /* ============================================================
     Input refs (for autofocus)
  ============================================================ */

  const inputRefs =
    useRef<Record<number, HTMLInputElement | null>>({});

  const activeStep =
    execution.steps.find(s => s.status === "active");

  useExecutionFocus({
    inputRefs,
    activeIndex: activeStep?.index ?? null,
    editingIndex: editingStep,
  });

  /* ============================================================
     Render
  ============================================================ */

  return (
    <>

      <Table.Root className="execution-table">

        <ExecutionHeader
          cutMode={cutMode}
          onCutModeChange={setCutMode}
        />

        <Table.Body>

          {execution.steps.map(step => {

            const draft =
              drafts[step.index] ??
              step.measurement.value ??
              "";

            return (

              <ExecutionTableRow
                key={step.index}
                step={step}
                cutMode={cutMode}
                decimals={decimals}
                draft={draft}
                error={errors[step.index]}
                editingStep={editingStep}
                inputRefs={inputRefs}
                onDraftChange={updateDraft}
                onConfirm={confirmEdit}
                onStartEdit={startEdit}
                onCancelEdit={cancelEdit}
                finished={finished}
                
              />

            );
          })}

        </Table.Body>

      </Table.Root>
    </>
  );
}

/* ============================================================
   Header (internal component)
============================================================ */

type CutMode = "deltaD" | "ae";

type HeaderProps = {
  cutMode: CutMode;
  onCutModeChange(mode: CutMode): void;
};

function ExecutionHeader({
  cutMode,
  onCutModeChange,
}: HeaderProps) {

  return (
    <Table.Head>

      <Table.Row>

        <Table.HeaderCell align="left">
          Step
        </Table.HeaderCell>

        <Table.HeaderCell align="left">
          Start Ø
        </Table.HeaderCell>

        <TableHeaderSelect
          value={cutMode}
          onChange={onCutModeChange}
          options={[
            { value: "deltaD", label: "ΔD" },
            { value: "ae", label: "ae" },
          ]}
          align="left"
        />

        <Table.HeaderCell align="left">
          Measurement
        </Table.HeaderCell>

        <Table.HeaderCell align="center" />

      </Table.Row>

    </Table.Head>
  );
}