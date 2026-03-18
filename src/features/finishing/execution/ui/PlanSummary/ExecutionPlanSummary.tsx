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
      title="Plan summary"
      actions={
        <button
          type="button"
          className="execution-edit-link"
          onClick={onEdit}
        >
          ← Edit plan
        </button>
      }
    >
      <dl className="execution-summary">

        <div className="execution-summary-item">
          <dt className="execution-summary-label">Mode</dt>
          <dd className="execution-summary-value">{mode}</dd>
        </div>

        <div className="execution-summary-item">
          <dt className="execution-summary-label">Start Ø</dt>
          <dd className="execution-summary-value">
            {startDiameter} mm
          </dd>
        </div>

        <div className="execution-summary-item">
          <dt className="execution-summary-label">Target Ø</dt>
          <dd className="execution-summary-value">
            {targetDiameter} mm
          </dd>
        </div>

      </dl>
    </Panel>
  );
}