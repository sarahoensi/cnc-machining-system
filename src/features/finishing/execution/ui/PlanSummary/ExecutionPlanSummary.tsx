// features/finishing/execution/ui/PlanSummary/ExecutionPlanSummary.tsx

import "./ExecutionPlanSummary.css";

type Props = {
  startDiameter: string
  targetDiameter: string
  mode: "Inner" | "Outer"
}

export function ExecutionPlanSummary({
  startDiameter,
  targetDiameter,
  mode,
}: Props) {

  return (
    <div className="execution-summary">

      <h3 className="execution-summary-title">
        Plan summary
      </h3>

      <div className="execution-summary-grid">

        <div>
          <strong>Mode:</strong> {mode}
        </div>

        <div>
          <strong>Start Ø:</strong> {startDiameter} mm
        </div>

        <div>
          <strong>Target Ø:</strong> {targetDiameter} mm
        </div>

      </div>

    </div>
  )
}