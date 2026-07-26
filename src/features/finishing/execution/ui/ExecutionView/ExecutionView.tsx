// finishing/execution/ui/ExecutionView/ExecutionView.tsx

import type { ExecutionState } from "@shared/execution";
import type { FinishingStepData } from "../../domain/mapExecution";

import { ExecutionPlanSummary } from "../PlanSummary";
//import { EditPlanButton } from "../EditPlanButton";
import { FinishingExecutionTable } from "../ExecutionTable";
import { ExecutionFinishedNotice } from "../ExecutionFinishedNotice/ExecutionFinishedNotice";

import { PageShell } from "@shared/ui/page/PageShell";
import { Stack } from "@shared/ui/primitives/Stack/Stack";

import "./ExecutionView.css";

type Props = {
  execution: ExecutionState<FinishingStepData>;

  summary: {
    mode: "Inner" | "Outer";
    startDiameter: string;
    targetDiameter: string;
  };

  onRegisterMeasurement(
    step: number,
    measurement: number
  ): Promise<void>;

  onEditPlan(): void;
  onReset(): void;
};

export function ExecutionView({
  execution,
  summary,
  onRegisterMeasurement,
  onEditPlan,
  onReset,
}: Props) {

  const finished =
    execution.activeIndex === execution.steps.length;

  return (
    <PageShell className="execution-view">
      <Stack className="execution-view-stack">
        <div className="execution-view-header">
          <ExecutionPlanSummary
            {...summary}
            onEdit={onEditPlan}
          />
        </div>

        <div className="execution-view-content">
        <FinishingExecutionTable
          execution={execution}
          onRegisterMeasurement={onRegisterMeasurement}
        />
        </div>

        {finished ? (
          <div className="execution-view-footer">
            <ExecutionFinishedNotice
              onCreateNewPlan={onReset}
            />
          </div>
        ) : null}
      </Stack>
    </PageShell>
  );
}

