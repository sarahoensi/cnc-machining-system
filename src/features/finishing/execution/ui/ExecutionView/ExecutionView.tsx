// finishing/execution/ui/ExecutionView/ExecutionView.tsx

import type { ExecutionState } from "@shared/execution";
import type { FinishingStepData } from "../../domain/mapExecution";

import { ExecutionPlanSummary } from "../PlanSummary";
//import { EditPlanButton } from "../EditPlanButton";
import { FinishingExecutionTable } from "../ExecutionTable";
import { ExecutionFinishedNotice } from "../ExecutionFinishedNotice/ExecutionFinishedNotice";

import { StackedLayout } from "@shared/ui/layout/page/StackedLayout/StackedLayout";

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
    <StackedLayout
      className="execution-view"

     header={
  <ExecutionPlanSummary
    {...summary}
    onEdit={onEditPlan}
  />
}

      content={
        <FinishingExecutionTable
          execution={execution}
          onRegisterMeasurement={onRegisterMeasurement}
        />
      }

      footer={
        finished ? (
          <ExecutionFinishedNotice
            onCreateNewPlan={onReset}
          />
        ) : null
      }
    />
  );
}