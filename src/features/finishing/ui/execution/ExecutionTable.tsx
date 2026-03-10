// features/finishing/ui/execution/ExecutionTable.tsx

import { useState } from "react";

import { Table } from "@shared/ui/components/table/Table/Table";
import { ExecutionNumberField } from "@shared/ui/components/execution/ExecutionNumberField";
import { RegisterButton } from "@shared/ui/components/primitives/Button/Button";

import type { ExecutionState } from "@shared/execution";
import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";
import { formatNumber } from "@shared/ui/format/formatNumber";

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
  ): void;
};

/* ============================================================
   Component
============================================================ */

export function FinishingExecutionTable({
  execution,
  onRegisterMeasurement,
}: Props) {

  const { decimals } = useDisplaySettings();

  /* ----------------------------------------------------------
     Local drafts (user input before submit)
  ---------------------------------------------------------- */

  const [drafts, setDrafts] =
    useState<Record<number, string>>({});

  function updateDraft(
    step: number,
    value: string
  ) {
    setDrafts(d => ({
      ...d,
      [step]: value,
    }));
  }

  /* ----------------------------------------------------------
     Render
  ---------------------------------------------------------- */

  return (

    <Table.Root>

      {/* ------------------------------------------- */}
      {/* Header                                      */}
      {/* ------------------------------------------- */}

      <Table.Head>

        <Table.Row>

          <Table.HeaderCell>
            Step
          </Table.HeaderCell>

          <Table.HeaderCell align="right">
            Start Ø
          </Table.HeaderCell>

          <Table.HeaderCell align="right">
            ΔD
          </Table.HeaderCell>

          <Table.HeaderCell align="right">
            Measurement
          </Table.HeaderCell>

          <Table.HeaderCell align="center" />

        </Table.Row>

      </Table.Head>

      {/* ------------------------------------------- */}
      {/* Body                                        */}
      {/* ------------------------------------------- */}

      <Table.Body>

        {execution.steps.map(step => {

          const draft =
            drafts[step.index] ??
            step.measurement.value;

          return (

            <Table.Row
              key={step.index}
              isActive={step.status === "active"}
            >

              {/* Step index */}

              <Table.Cell>
                {step.index}
              </Table.Cell>

              {/* Start diameter */}

              <Table.Cell align="right">

                <ExecutionNumberField
                  state={step.status}
                  value={formatNumber(step.data.startDiameter, decimals)}
                  readonly
                />

              </Table.Cell>

              {/* Delta */}

              <Table.Cell align="right">

                <ExecutionNumberField
                  state={step.status}
                  value={formatNumber(step.data.deltaD, decimals)}
                  readonly
                />

              </Table.Cell>

              {/* Measurement */}

              <Table.Cell align="right">

                <ExecutionNumberField
                  state={step.status}

                  value={draft}

                  placeholder={formatNumber(
                    step.data.expectedDiameter,
                    decimals
                  )}

                  onChange={(v) =>
                    updateDraft(step.index, v)
                  }

                  onSubmit={() =>
                    onRegisterMeasurement(
                      step.index,
                      Number(draft)
                    )
                  }

                />

              </Table.Cell>

              {/* Action column */}

              <Table.Cell align="center">

                {step.status === "active" && (

                  <RegisterButton
                    disabled={!draft}
                    onClick={() =>
                      onRegisterMeasurement(
                        step.index,
                        Number(draft)
                      )
                    }
                  />

                )}

              </Table.Cell>

            </Table.Row>

          );

        })}

      </Table.Body>

    </Table.Root>
  );
}