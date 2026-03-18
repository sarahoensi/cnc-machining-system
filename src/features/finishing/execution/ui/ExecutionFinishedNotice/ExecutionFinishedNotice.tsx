// ExecutionFinisheNotice.tsx

import { Panel } from "@shared/ui/layout/container/Panel/Panel";

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
        <button
          type="button"
          className="primary-button"
          onClick={onCreateNewPlan}
        >
          Create new plan
        </button>
      }
    >
      <p>
        All measurements are registered.
      </p>
    </Panel>
  );
}