// ExecutionFinisheNotice.tsx

type Props = {
  onCreateNewPlan(): void;
};

export function ExecutionFinishedNotice({
  onCreateNewPlan,
}: Props) {
  return (
    <div className="execution-finished">

      <div className="execution-finished-text">
        <strong>Finishing complete</strong>
        <p>
          All measurements are registered.
        </p>
      </div>

      <button
        type="button"
        className="primary-button"
        onClick={onCreateNewPlan}
      >
        Create new plan
      </button>

    </div>
  );
}