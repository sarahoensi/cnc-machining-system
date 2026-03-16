// finishing/execution/ui/ExecutionView/ExecutionView.tsx

import type { ExecutionState } from "@shared/execution";
import type { FinishingStepData } from "../../domain/mapExecution";

import { ExecutionPlanSummary } from "../PlanSummary";
import { EditPlanButton } from "../EditPlanButton";
import { FinishingExecutionTable } from "../ExecutionTable";

import "./ExecutionView.css";

type Props = {
  execution: ExecutionState<FinishingStepData>;

  summary: {
    mode: "Inner" | "Outer";
    startDiameter: string;
    targetDiameter: string;
    cuts?: string;
    radialEngagement?: string;
  };

  onRegisterMeasurement(
    step: number,
    measurement: number
  ): Promise<void>;

  onEditPlan(): void;
};

export function ExecutionView({
  execution,
  summary,
  onRegisterMeasurement,
  onEditPlan,
}: Props) {
  return (
    <div className="execution-view">

      <ExecutionPlanSummary {...summary} />

      <EditPlanButton onClick={onEditPlan} />

      <FinishingExecutionTable
        execution={execution}
        onRegisterMeasurement={onRegisterMeasurement}
      />

    </div>
  );
}