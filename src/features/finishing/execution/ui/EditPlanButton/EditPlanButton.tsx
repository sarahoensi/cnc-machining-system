// EditPlanButton.tsx

import "./EditPlanButton.css";

type Props = {
  onClick(): void;
};

export function EditPlanButton({ onClick }: Props) {
  return (
    <div className="execution-header">
      <button
        type="button"
        className="execution-edit-link"
        onClick={onClick}
      >
        ← Edit plan
      </button>
    </div>
  );
}