// ExecutionFinisheNotice.tsx

import { Panel } from "@shared/ui/layout/container/Panel/Panel";
import { Button } from "@shared/ui/primitives/Button/Button";

type Props = {
  onCreateNewPlan(): void;
};

export function ExecutionFinishedNotice({
  onCreateNewPlan,
}: Props) {
  return (
    <Panel
      title="Finishing complete"
      actions={
        <Button size="small" variant="primary" onClick={onCreateNewPlan}>
          Create new plan
        </Button>
      }
    >
      <p className="execution-finished-text">
        All measurements are registered.
      </p>
    </Panel>
  );
}