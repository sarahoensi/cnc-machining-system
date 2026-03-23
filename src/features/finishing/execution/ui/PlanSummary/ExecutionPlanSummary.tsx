// ExecutionPlanSummary.tsx

import { Panel } from "@shared/ui/layout/container/Panel/Panel";
import { FieldDisplay } from "@shared/ui/components/display/FieldDisplay";
import { Button } from "@shared/ui/primitives/Button/Button";
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
        <Button variant="link" onClick={onEdit}>
          ← Edit plan
        </Button>
      }
    >
      <div className="execution-summary">

        <FieldDisplay label="Mode:" value={mode} align="left" />

        <div className="execution-summary-row">
          <FieldDisplay label="Start Ø:" value={startDiameter} />
          <FieldDisplay label="Target Ø:" value={targetDiameter} />
        </div>

      </div>
    </Panel>
  );
}