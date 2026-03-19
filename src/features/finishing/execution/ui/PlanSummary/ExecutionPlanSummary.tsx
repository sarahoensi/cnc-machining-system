// ExecutionPlanSummary.tsx

import { Panel } from "@shared/ui/layout/container/Panel/Panel";
import "./ExecutionPlanSummary.css";

type Props = {
  startDiameter: string;
  targetDiameter: string;
  mode: "Inner" | "Outer";

  onEdit(): void;
};

export function ExecutionPlanSummary({
  startDiameter,
  targetDiameter,
  mode,
  onEdit,
}: Props) {

 return (
    <Panel
      className="panel-secondary"
      title="Plan summary"
      actions={
        <button
          type="button"
          className="execution-edit-link"
          onClick={onEdit}
        >
          Edit plan
        </button>
      }
    >
      <div className="execution-summary">

        <div className="execution-summary-mode">
          <span className="label">Mode:</span>
          <span>{mode}</span>
        </div>

        <div className="execution-summary-row">

          <span className="execution-summary-item">
            <span className="label">Start Ø:</span>
            <span>{startDiameter}</span>
          </span>

          <span className="execution-summary-item">
            <span className="label">Target Ø:</span>
            <span>{targetDiameter}</span>
          </span>

        </div>

      </div>
    </Panel>
  );
}