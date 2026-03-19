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
      Edit plan
    </button>
  }
>
 <div className="execution-summary">

  <div className="execution-summary-mode">
    <span className="label">Mode:</span> {mode}
  </div>

  <div className="execution-summary-row">
    <span className="execution-summary-item">
      <span className="label">Start Ø:</span> {startDiameter}
    </span>

    <span className="execution-summary-item">
      <span className="label">Target Ø:</span> {targetDiameter}
    </span>
  </div>

</div>
</Panel>
  );
}